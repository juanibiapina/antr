extern crate antr;

extern crate clap;
extern crate env_logger;
extern crate git2;
extern crate log;
extern crate notify;
extern crate notify_debouncer_mini;

use clap::Parser;
use git2::Repository;
use log::{warn, error, debug};
use notify::RecursiveMode;
use notify_debouncer_mini::DebouncedEvent;
use notify_debouncer_mini::new_debouncer;
use std::env;
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use antr::error::Error;

struct ShellCommand {
    command: String,
    args: Vec<String>,
}

enum Event {
    Trigger(Result<Vec<DebouncedEvent>, notify::Error>),
    Manual
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(help = "Command to run", required = true)]
    command: String,

    #[arg(long, help = "Exit after running the command once")]
    runonce: bool,

    #[arg(help = "Command with args", trailing_var_arg = true, allow_hyphen_values = true, num_args = ..)]
    args: Vec<String>,
}

fn main() {
    // init env logger
    env_logger::init();

    match old_main() {
        Ok(()) => {},
        Err(e) => handle_error(e),
    }
}

fn old_main() -> Result<(), Error> {
    // parse command line arguments
    let cli = Cli::parse();

    let shell_command = Arc::new(ShellCommand {
        command: cli.command,
        args: cli.args,
    });

    // determine the current directory
    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => return Err(Error::InvalidCurrentDirectory(std::rc::Rc::new(e))),
    };

    debug!("Opening git repository...");
    let repo = match Repository::open(&current_dir) {
        Ok(repo) => Some(repo),
        Err(e) => {
            warn!("git error: {:?}", e);
            warn!("proceeding without git repository");
            None
        },
    };

    // Main channel for receiving all events
    let (main_tx, main_rx) = channel();

    // Initialize watcher
    let _debouncer = initialize_watcher(&current_dir, repo.as_ref(), main_tx.clone())?;
    let _manual_thread = initialize_manual(main_tx.clone());

    let running = Arc::new(Mutex::new(false));

    debug!("Listening for changes...");
    while let Ok(event) = main_rx.recv() {
        let should_run = match event {
            Event::Manual => {
                debug!("Manual event received");
                true
            }

            Event::Trigger(events) => {
                debug!("Processing events from Event::Trigger...");
                let should_run = match repo {
                    Some(ref repo) => {
                        match events {
                            Ok(events) => {
                                let mut result = false;

                                for event in events.iter() {
                                    debug!("event path: {:?}", event.path);
                                    let should_ignore = match repo.status_should_ignore(&event.path) {
                                        Ok(value) => value,
                                        Err(e) => {
                                            error!("git ignore error: {:?}", e);
                                            false
                                        }
                                    };

                                    // if one file cannot be ignored, we already know we need to run
                                    if ! should_ignore {
                                        result = true;
                                        continue
                                    }
                                }

                                result
                            },
                            Err(e) => {
                                error!("watch error: {:?}", e);
                                true
                            }
                        }
                    },
                    None => {
                        true
                    }
                };

                if should_run {
                    debug!("changes detected");
                } else {
                    debug!("ignoring changes");
                }

                should_run
            }
        };

        debug!("Attempting to run command...");
        if should_run {
            let mut local_running = running.lock().unwrap();

            if ! *local_running {
                debug!("Running command...");
                *local_running = true;
                let thread_command = shell_command.clone();
                let thread_running = running.clone();

                thread::spawn(move|| {
                    Command::new("clear").status().unwrap();
                    let mut command = Command::new(&thread_command.command);

                    command.args(&thread_command.args);

                    let exit_status = command.status().unwrap();
                    println!("");
                    println!("{}", exit_status);

                    if cli.runonce {
                        std::process::exit(exit_status.code().unwrap());
                    }

                    let mut local_running = thread_running.lock().unwrap();
                    *local_running = false;
                });
            } else {
                debug!("Command already running, skipping run.");
            }
        }
    }

    Ok(())
}

fn initialize_watcher(
    current_dir: &std::path::Path,
    repo: Option<&Repository>,
    main_tx: std::sync::mpsc::Sender<Event>,
) -> Result<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>, Error> {
    // collect entries in the current directory
    let entries = match current_dir.read_dir() {
        Ok(entries) => entries,
        Err(e) => return Err(Error::CantReadCurrentDirectory(std::rc::Rc::new(e))),
    };

    // create a channel to communicate with the debouncer
    let (debouncer_tx, debouncer_rx) = channel();

    // initialize the debouncer
    let mut debouncer = match new_debouncer(Duration::from_secs(1), debouncer_tx) {
        Ok(debouncer) => debouncer,
        Err(e) => return Err(Error::DebouncerInitializationError(std::rc::Rc::new(e))),
    };

    // setup watchers for entries in the current directory
    debug!("Setting up watchers for entries in the current directory...");
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if let Some(repo) = repo {
                    let should_ignore = match repo.status_should_ignore(&path) {
                        Ok(value) => value,
                        Err(e) => {
                            error!("git ignore error: {:?}", e);
                            false
                        }
                    };

                    if should_ignore {
                        debug!("Ignoring path: {:?}", path);
                        continue;
                    }
                }

                debug!("Watching path: {:?}", path);
                match debouncer.watcher().watch(&path, RecursiveMode::Recursive) {
                    Ok(()) => {},
                    Err(e) => {
                        return Err(Error::WatcherError(path.to_owned(), std::rc::Rc::new(e)));
                    },
                };
            },
            Err(e) => return Err(Error::ReadEntryError(std::rc::Rc::new(e))),
        }
    }

    // Spawn a thread to forward triggers to the main channel
    let main_tx_clone = main_tx.clone();
    thread::spawn(move || {
        while let Ok(events) = debouncer_rx.recv() {
            main_tx_clone.send(Event::Trigger(events)).expect("main_tx send failed");
        }
    });

    Ok(debouncer)
}

fn initialize_manual(main_tx: std::sync::mpsc::Sender<Event>) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            debug!("Waiting for manual event...");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            debug!("Manual event produced");
            main_tx.send(Event::Manual).expect("Failed to send Event::Manual");
        }
    })
}

fn handle_error(error: Error) -> ! {
    match error {
        Error::InvalidCurrentDirectory(e) => {
            println!("antr: unable to determine current directory: {:?}", e);
        }
        Error::CantReadCurrentDirectory(e) => {
            println!("antr: unable to read current directory: {:?}", e);
        }
        Error::DebouncerInitializationError(e) => {
            println!("antr: unable to initialize debouncer: {:?}", e);
        }
        Error::WatcherError(path, e) => {
            println!("antr: unable to watch path {:?}: {:?}", path, e);
        }
        Error::ReadEntryError(e) => {
            println!("antr: unable to read entry: {:?}", e);
        }
    }

    std::process::exit(1);
}

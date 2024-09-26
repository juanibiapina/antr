extern crate clap;
extern crate env_logger;
extern crate git2;
extern crate log;
extern crate notify;
extern crate notify_debouncer_mini;

use clap::Parser;
use git2::Repository;
use log::{info, error, debug};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct ShellCommand {
    command: String,
    args: Vec<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(help = "Command to run", required = true)]
    command: String,

    #[arg(help = "Command with args", trailing_var_arg = true, num_args = ..)]
    args: Vec<String>,
}

fn die(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1);
}

fn main() {
    // init env logger
    env_logger::init();

    // parse command line arguments
    let cli = Cli::parse();

    let shell_command = Arc::new(ShellCommand {
        command: cli.command,
        args: cli.args,
    });

    // determine the current directory
    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(_) => die("could not determine current directory"),
    };

    // create a channel to communicate with the debouncer
    let (tx, rx) = channel();

    // initialize the debouncer
    let mut debouncer = match new_debouncer(Duration::from_secs(1), tx) {
        Ok(debouncer) => debouncer,
        Err(_) => {
            die("antr: unable to initialize debouncer");
        },
    };

    info!("Watching directory: {:?}", current_dir);
    match debouncer.watcher().watch(Path::new(&current_dir), RecursiveMode::Recursive) {
        Ok(()) => {},
        Err(e) => {
            println!("{:?}", e);
            die("antr: unable to watch current directory");
        },
    };

    info!("Opening git repository...");
    let repo = Repository::open(current_dir);

    let running = Arc::new(Mutex::new(false));

    info!("Listening for changes...");
    while let Ok(events) = rx.recv() {
        info!("Processing events...");
        let should_run = match repo {
            Ok(ref repo) => {
                match events {
                    Ok(events) => {
                        let mut result = false;

                        for event in events.iter() {
                            debug!("event path: {:?}", event.path);
                            let should_ignore = match repo.status_should_ignore(&event.path) {
                                Ok(value) => value,
                                Err(e) => {
                                    error!("git ignore error: {:?}", e);
                                    true
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
            Err(ref e) => {
                error!("git error: {:?}", e);
                true
            }
        };

        if should_run {
            info!("changes detected");
        } else {
            info!("ignoring changes");
            continue;
        }

        let mut local_running = running.lock().unwrap();
        if ! *local_running {
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

                let mut local_running = thread_running.lock().unwrap();

                *local_running = false;
            });
        }
    }
}

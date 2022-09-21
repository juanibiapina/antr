extern crate notify;
extern crate notify_debouncer_mini;
extern crate git2;

use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;
use git2::Repository;

struct ShellCommand {
    command: String,
    args: Vec<String>,
}

fn die(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1);
}

fn main() {
    let mut argv = env::args().skip(1);

    let command = match argv.next() {
        Some(command) => command,
        None => {
            die("antr: no command passed");
        },
    };

    let mut args = Vec::new();

    for arg in argv {
        args.push(arg)
    }

    let shell_command = Arc::new(ShellCommand {
        command: command,
        args: args,
    });

    let (tx, rx) = channel();

    let mut debouncer = match new_debouncer(Duration::from_secs(1), None, tx) {
        Ok(debouncer) => debouncer,
        Err(_) => {
            die("antr: unable to initialize debouncer");
        },
    };

    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(_) => die("could not determine current directory"),
    };

    match debouncer.watcher().watch(Path::new(&current_dir), RecursiveMode::Recursive) {
        Ok(()) => {},
        Err(_) => {
            die("antr: unable to watch current directory");
        },
    };

    let repo = Repository::open(current_dir);

    let running = Arc::new(Mutex::new(false));

    while let Ok(events) = rx.recv() {
        let ignore = match repo {
            Ok(ref repo) => {
                match events {
                    Ok(events) => {
                        let mut result = false;

                        for event in events.iter() {
                            if event.path.exists() {
                                result = result || match repo.status_should_ignore(&event.path) {
                                    Ok(value) => value,
                                    Err(_) => false,
                                };
                            }
                        }

                        result
                    },
                    Err(_) => false,
                }
            },
            Err(_) => false,
        };

        if ignore {
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

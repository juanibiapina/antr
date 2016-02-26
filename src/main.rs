extern crate notify;
extern crate git2;

use std::env;
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

use notify::{RecommendedWatcher, Watcher};
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

    let mut watcher: RecommendedWatcher = match Watcher::new(tx) {
        Ok(watcher) => watcher,
        Err(_) => die("antr: error starting file system watcher"),
    };

    match watcher.watch(".") {
        Ok(()) => {},
        Err(_) => {
            die("antr: unable to watch current directory");
        },
    }

    let repo = Repository::open(".");

    let running = Arc::new(Mutex::new(false));

    while let Ok(event) = rx.recv() {
        let ignore = match repo {
            Ok(ref repo) => {
                match event.path {
                    Some(path_buf) => {
                        match repo.status_should_ignore(path_buf.as_path()) {
                            Ok(value) => value,
                            Err(_) => false,
                        }
                    },
                    None => false,
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

extern crate notify;

use std::sync::mpsc::channel;
use std::process::Command;
use std::env;

use notify::{RecommendedWatcher, Watcher};

fn main() {
    let mut args = env::args().skip(1);

    let command = match args.next() {
        Some(command) => command,
        None => {
            panic!("antr: no command passed");
        },
    };

    let mut command = Command::new(command);

    while let Some(arg) = args.next() {
        command.arg(arg);
    }

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = match Watcher::new(tx) {
        Ok(watcher) => watcher,
        Err(_) => panic!("antr: error starting file system watcher"),
    };

    match watcher.watch(".") {
        Ok(()) => {},
        Err(_) => {
            panic!("antr: unable to watch current directory");
        },
    }

    while let Ok(_) = rx.recv() {
        Command::new("clear").status().unwrap();
        let exit_status = command.status().unwrap();
        println!("");
        println!("{}", exit_status);
    }
}

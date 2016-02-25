extern crate notify;

use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::process::Command;

fn main() {
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
        let exit_status = Command::new("ls").status().unwrap();
        println!("");
        println!("exit status: {}", exit_status);
    }
}

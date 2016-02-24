extern crate notify;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            watcher.watch(".").unwrap();

            match rx.recv() {
                _ => println!("Recv.")
            }
        },
        Err(_) => {
            panic!("antr: error starting file system watcher");
        },
    }
}

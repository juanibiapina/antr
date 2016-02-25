extern crate notify;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            watcher.watch(".").unwrap();

            while let Ok(event) = rx.recv() {
                println!("Received event: {:?}", event.op.unwrap());
            }
        },
        Err(_) => {
            panic!("antr: error starting file system watcher");
        },
    }
}

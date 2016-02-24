extern crate notify;

use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut w: Result<RecommendedWatcher, Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher.watch(".");

            // You'll probably want to do that in a loop. The type to match for is
            // notify::Event, look at src/lib.rs for details.
            match rx.recv() {
                _ => println!("Recv.")
            }
        },
        Err(e) => println!("Error")
    }
}

extern crate notify;
extern crate walkdir;

use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

use walkdir::WalkDir;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};

fn watch(path: &Path) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));
    try!(watcher.watch(path, RecursiveMode::Recursive));
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub fn sync(path: &Path) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

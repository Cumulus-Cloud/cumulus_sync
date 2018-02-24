extern crate notify;
extern crate walkdir;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

use walkdir::WalkDir;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};

mod errors;
use errors::*;

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

pub fn sync(path: &Path, server_url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = File::open(&path)?;

      
        println!("{}", entry.path().display());
    }
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
    Ok(())
}

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
extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod errors;
pub mod api;

// use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::sync::mpsc::channel;

// use walkdir::WalkDir;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use api::{CumulusApi, Auth};
use std::path::PathBuf;
use errors::*;

#[derive(Debug)]
pub struct Options {
    pub server_url: String,
    pub auth: Auth,
    pub folder: PathBuf,
}

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

pub fn sync(options: Options) -> Result<()> {
    println!("Opt {:?}", options);
    let client = reqwest::Client::new();
    let auth = Auth::new(&options.auth.login, &options.auth.password);
    let api = CumulusApi::create(options.server_url, &auth)?;
    let root = api.fs_node("/")?;
    println!("Root {:?}", root);

    /*
    let file_type = folder.metadata()?.file_type();
    if file_type.is_dir() {
      println!("Dir {:?}", file_type.is_dir());
      //let _ = cumulus_sync::sync(&opt.folder, "http://localhost:9000");
    }
    */

    /*
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = File::open(&path)?;
        println!("{}", entry.path().display());
    }
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
    */
    Ok(())
}

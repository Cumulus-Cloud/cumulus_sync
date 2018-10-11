// use std::fs::File;
// use std::path::Path;
// use std::time::Duration;
// use std::sync::mpsc::channel;

// use walkdir::WalkDir;
// use notify::{RecommendedWatcher, Watcher, RecursiveMode};

// use std::path::PathBuf;

// fn watch(path: &Path) -> notify::Result<()> {
//     let (tx, rx) = channel();
//     let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));
//     try!(watcher.watch(path, RecursiveMode::Recursive));
//     loop {
//         match rx.recv() {
//             Ok(event) => println!("{:?}", event),
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     }
// 

fn run() {
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
}

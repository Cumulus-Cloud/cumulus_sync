extern crate clap;
extern crate cumulus_sync;

use std::path::Path;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Cumulus Sync CLI")
        .version("0.0.1")
        .author("Wadjetz <egor.neon@gmail.com>")
        .about("Files sync for Cumulus")
        .arg(
            Arg::with_name("watch")
                .short("w")
                .long("watch")
                .value_name("Directory")
                .required(true)
                .validator(|value| {
                    let path = Path::new(&value);
                    if path.exists() {
                        if path.is_dir() {
                            Ok(())
                        } else {
                            Err("is not a directory".to_owned())
                        }
                    } else {
                        Err("Directory is not existe".to_owned())
                    }
                })
        )
        .get_matches();

    if let Some(watch_path) = matches.value_of("watch") {
        let path = Path::new(watch_path);
        cumulus_sync::sync(path);
    }
}

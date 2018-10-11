extern crate notify;
extern crate reqwest;
extern crate walkdir;
#[macro_use]
extern crate serde_derive;
extern crate serde;
// #[macro_use]
// extern crate serde_json;
extern crate chrono;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;

mod api;

use std::path::Path;
use std::fs::File;
use reqwest::{Client, Error};

use api::cumulus_api::CumulusApi;

pub fn sync(server: String, login: String, password: String) -> Result<(), Error> {
    env_logger::init();
    info!("Sync");
    let api = CumulusApi::new(Client::new(), server).authenticate(login, password)?;
    let root = api.fs_node("/")?;
    info!("Root {:?}", root);

    let path = Path::new("/Users/egor/projects/cumulus_sync/Cargo.toml");
    if let Ok(file) = File::open(path) {
        let fs_node = api.upload("Cargo.toml", file)?;
        info!("Uploaded {:?}", fs_node);
    }
    Ok(())
}

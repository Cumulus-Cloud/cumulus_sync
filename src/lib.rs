mod api;

use std::path::Path;
use std::fs::File;
use reqwest::{Client, Error};
use log::info;

use crate::api::cumulus_api::CumulusApi;

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

use std::path::{Path, PathBuf};
use std::fs::File;

use reqwest::{Client, Error};
use log::{error, info};
use walkdir::WalkDir;

mod api;

use crate::api::CumulusApi;

pub fn upload(folder: PathBuf, target: String, server: String, login: String, password: String) -> Result<(), Error> {
    info!("Upload {:?}", folder.as_path().to_str());
    let api = CumulusApi::new(Client::new(), server).authenticate(login, password)?;

    let root_fs_node = api.fs_node(&target)?;

    WalkDir::new(&folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .for_each(|entry| {
            let path = entry.path();
            println!("path = {:?}", &get_relative_path(&folder, path));
            if let Ok(file) = File::open(path) {
                match api.upload(&root_fs_node, &get_relative_path(&folder, path), file) {
                    Ok(fs_node) => info!("Uploaded {:?}", fs_node.get_path()),
                    Err(err) => error!("Upload Error {:?}", err),
                }
                
            }
        });
    Ok(())
}

fn get_relative_path(folder: &Path, path: &Path) -> String {
    let folder = folder.to_string_lossy().into_owned();
    let path = path.to_string_lossy().into_owned();
    path.replace(&folder, "")
}

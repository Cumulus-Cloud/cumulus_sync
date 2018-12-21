use std::path::PathBuf;
use structopt_derive::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cumulus_sync", about = "Files sync for Cumulus")]
pub enum Cli {
    #[structopt(name = "upload", about = "Upload files")]
    Upload {
        #[structopt(short = "f", long = "folder", parse(from_os_str))]
        folder: PathBuf,

        #[structopt(short = "t", long = "target")]
        target: String,

        #[structopt(flatten)]
        auth_cli: AuthCli,
    }
}

#[derive(Debug, StructOpt)]
pub struct AuthCli {
    #[structopt(short = "l", long = "login")]
    pub login: String,

    #[structopt(short = "p", long = "password")]
    pub password: String,

    #[structopt(short = "s", long = "server")]
    pub server: String,
}

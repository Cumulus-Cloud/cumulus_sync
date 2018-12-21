extern crate cumulus_sync;

mod cli;

use crate::cli::Cli;
use structopt::StructOpt;

fn main() {
    std::env::set_var("RUST_LOG", "cumulus_sync=debug,errors=debug");
    env_logger::init();
    let cli = Cli::from_args();

    match cli {
        Cli::Upload { auth_cli, folder, target } => {
            if let Err(error) = cumulus_sync::upload(folder, target, auth_cli.server, auth_cli.login, auth_cli.password) {
                println!("Error {:?}", error);
            }
        }
    }
}

extern crate cumulus_sync;

mod cli;

use crate::cli::Cli;
use structopt::StructOpt;

use cumulus_sync::upload;

fn main() {
    std::env::set_var("RUST_LOG", "cumulus_sync=debug,errors=debug");
    env_logger::init();
    let cli = Cli::from_args();

    let result = match cli {
        Cli::Upload { auth_cli, folder, target } => {
            upload(folder, target, auth_cli.server, auth_cli.login, auth_cli.password)
        },
    };

    if let Err(error) = result {
        println!("Error {:?}", error);
    }
}

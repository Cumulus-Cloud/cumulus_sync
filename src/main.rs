extern crate cumulus_sync;

mod cli;

use crate::cli::Cli;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    if let Err(error) = cumulus_sync::sync(cli.server, cli.login, cli.password) {
        println!("Error {:?}", error);
    }
}

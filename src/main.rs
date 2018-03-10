extern crate cumulus_sync;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod cli;

use structopt::StructOpt;
use cli::Cli;
use cumulus_sync::errors::Error;

fn start(opt: Cli) -> Result<(), Error> {
    Ok(cumulus_sync::sync(opt.into())?)
}

fn main() {
    let opt = Cli::from_args();
    if let Err(error) = start(opt) {
        println!("Error {:?}", error);
    }
}

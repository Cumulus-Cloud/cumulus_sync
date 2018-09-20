extern crate cumulus_sync;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod cli;

use cli::Cli;
use structopt::StructOpt;

fn main() {
    let opt = Cli::from_args();
    if let Err(error) = cumulus_sync::sync(opt.into()) {
        println!("Error {:?}", error);
    }
}

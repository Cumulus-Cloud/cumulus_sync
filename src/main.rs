extern crate cumulus_sync;
extern crate reqwest;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

mod cli;

use structopt::StructOpt;
use cli::Cli;

use std::fs::File;
use std::io::Error;

fn start(opt: Cli) -> Result<(), Error> {
    let folder = File::open(&opt.folder)?;
    let file_type = folder.metadata()?.file_type();
    if file_type.is_dir() {
      println!("Dir {:?}", file_type.is_dir());
      cumulus_sync::sync(&opt.folder, "http://localhost:9000");
    }
    println!("Opt {:?}", opt);
    Ok(())
}

fn main() {
    let opt = Cli::from_args();
    let _ = start(opt);
}

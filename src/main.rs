#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate structopt;

mod cli;
mod platform;
mod prepare;

use cli::Cli;
use platform::Platform;
use structopt::StructOpt;

fn main() {
    pretty_env_logger::init();
    match Cli::from_args() {
        Cli::Prepare(args) => prepare::main(&args),
    }
}

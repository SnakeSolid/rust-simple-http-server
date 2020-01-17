#[macro_use]
extern crate log;

mod options;
mod server;

use options::Options;
use structopt::StructOpt;

fn main() {
    env_logger::init();

    let options = Options::from_args();

    server::start(&options);
}

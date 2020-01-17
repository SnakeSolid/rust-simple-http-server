use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gantt-diagram")]
pub struct Options {
    #[structopt(
        short = "a",
        long = "address",
        name = "ADDR",
        help = "Listen on given address",
        default_value = "localhost"
    )]
    address: String,

    #[structopt(
        short = "p",
        long = "port",
        name = "PORT",
        help = "Listen on given port",
        default_value = "8080"
    )]
    port: u16,

    #[structopt(
        short = "d",
        long = "path",
        name = "PATH",
        help = "Path to server directory",
        default_value = ".",
        parse(from_os_str)
    )]
    path: PathBuf,
}

impl Options {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

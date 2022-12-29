mod cli;
mod errors;
mod config;
mod container;
mod ipc;
mod child;
mod hostname;
mod mounts;
mod namespaces;
mod capabilities;
mod syscalls;
mod resources;

#[macro_use] extern crate scan_fmt;

use errors::exit_with_retcode;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args));
        },
        Err(e) => {
            //log::error!("Error while parsing arguments:\n\t{}", e);
            exit_with_retcode(Err(e));
        }
    }

}
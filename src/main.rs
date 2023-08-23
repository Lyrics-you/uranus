#![windows_subsystem = "windows"]

pub mod assemble;
pub mod components;
pub mod pannel;
pub mod toast;
pub mod utils;

extern crate log;
extern crate log4rs;

use pannel::pannel_main;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    pannel_main().unwrap();
}

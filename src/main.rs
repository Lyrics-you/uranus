// without console
#![windows_subsystem = "windows"]

pub mod assemble;
pub mod components;
pub mod panel;
pub mod toast;
pub mod utils;

extern crate log;
extern crate log4rs;

use panel::pannel_main;

fn main() {
    // init log4rs by file "log4rs.yaml"
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    //  launch panel
    pannel_main().unwrap();
}

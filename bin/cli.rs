//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::StructOpt;
use error_safari::{interface::Safari, run};

fn main() {
    let args = Safari::parse();
    run(args);
}

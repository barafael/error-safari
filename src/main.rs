//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::{Parser, ValueHint};
use klask::Settings;
use std::path::PathBuf;

mod safari;

#[derive(Debug, Parser)]
#[clap(name = "Error Safari")]
/// Check out all the ways to define and handle errors in Rust!
pub struct Safari {
    /// Any input other than `fail` will succeed. Try 'fail-nice' also.
    input: String,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Subcommands also display help
    Unwrap {
        #[clap(long, parse(from_os_str), value_hint = ValueHint::AnyPath)]
        path: Option<PathBuf>,
    },
    Native {
        #[clap(long)]
        flag: bool,
        #[clap(short, parse(from_occurrences))]
        volume: i32,
    },
    Thiserror {
        #[clap(possible_values = &["One", "Two", "Three"])]
        choose_one: String,
    },
}

fn main() {
    klask::run_derived::<Safari, _>(Settings::default(), |safari| safari.run());
}

use crate::safari::{external, thiserror::Operation};
use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "Error Safari")]
/// Check out all the ways to define and handle errors in Rust!
pub struct Safari {
    /// Any input other than `fail` will succeed. Try 'fail-nice' also.
    pub input: String,

    /// CLI or not?
    #[clap(long, takes_value = false)]
    pub cli: bool,

    /// Error handling style to use.
    #[clap(subcommand)]
    pub subcommand: Style,
}

#[derive(Debug, Parser)]
pub enum Style {
    /// Unwrapping, YOLO
    Unwrap {
        #[clap(long, parse(from_os_str), value_hint = ValueHint::AnyPath)]
        path: Option<PathBuf>,
    },

    /// Explicitly defined, match-able error enums.
    Thiserror {
        #[clap(arg_enum)]
        choose_one: Option<Operation>,
    },

    /// Match-able error enums from other crates.
    ExternalCrate {
        #[clap(arg_enum)]
        alt: external::SomeCrate,
    },

    /// ¯\_(°ペ)_/¯ just some errors, bubbling up.
    Anyhow {
        #[clap(short, parse(from_occurrences))]
        choice: u32,
    },
}

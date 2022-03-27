//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::{Parser, ValueHint};
use klask::Settings;
use safari::external;
use std::{borrow::Cow, path::PathBuf};

mod safari;

#[derive(Debug, Parser)]
#[clap(name = "Error Safari")]
/// Check out all the ways to define and handle errors in Rust!
pub struct Safari {
    /// Any input other than `fail` will succeed. Try 'fail-nice' also.
    input: String,

    #[clap(long, takes_value = false)]
    cli: bool,

    #[clap(subcommand)]
    subcommand: Style,
}

#[derive(Debug, Parser)]
pub enum Style {
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
        #[clap(arg_enum)]
        choose_one: Option<Alternative>,
    },

    ExternalCrate {
        #[clap(arg_enum)]
        alt: external::Alternative,
    },
}

#[derive(Debug, Parser, Clone, clap::ArgEnum)]
pub enum Alternative {
    A,
    B,
    C,
}

fn main() {
    match Safari::try_parse() {
        Ok(s @ Safari { cli: true, .. }) => {
            safari::run(s);
        }
        _ => {
            let mut settings = Settings::default();
            settings.custom_font = Some(Cow::Borrowed(include_bytes!(r"../Roboto-Medium.ttf")));
            //settings.style.body_text_style = TextStyle::Heading;

            klask::run_derived::<Safari, _>(settings, safari::run);
        }
    };
}

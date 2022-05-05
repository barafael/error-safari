//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::StructOpt;
use cli::Safari;
use klask::Settings;
use std::borrow::Cow;

/// Command Line Interface.
mod cli;

/// Have a look around!
mod safari;

fn main() {
    if let Ok(s @ Safari { cli: true, .. }) = Safari::try_parse() {
        safari::run(s);
    } else {
        let mut settings = Settings::default();
        settings.custom_font = Some(Cow::Borrowed(include_bytes!(r"../Roboto-Medium.ttf")));

        klask::run_derived::<Safari, _>(settings, safari::run);
    };
}

//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::StructOpt;
use cli::Safari;
use klask::Settings;
use std::borrow::Cow;

mod cli;
mod safari;

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

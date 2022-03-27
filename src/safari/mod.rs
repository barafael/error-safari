use crate::{Safari, Style};

pub(crate) mod external;
mod thiserror;
mod unwrapping;

pub fn run(safari: Safari) {
    match safari.subcommand {
        Style::Unwrap { path: _path } => unwrapping::run(safari.input),
        Style::Thiserror { choose_one } => thiserror::run(safari.input, choose_one),
        Style::ExternalCrate { alt } => {
            external::run(safari.input, alt);
        }
        _ => println!("{:?}", safari),
    }
}

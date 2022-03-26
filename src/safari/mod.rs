use crate::{Safari, Style};

mod thiserror;
mod unwrapping;

pub fn run(safari: Safari) {
    match safari.subcommand {
        Style::Unwrap { path } => unwrapping::run(safari.input),
        Style::Thiserror { choose_one } => thiserror::run(safari.input, choose_one),
        _ => println!("{:?}", safari),
    }
}

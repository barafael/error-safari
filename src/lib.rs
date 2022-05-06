use interface::{Safari, Style};

/// Command Line Interface.
pub mod interface;

pub mod anyhow;
pub mod external;
pub mod thiserror;
mod unwrapping;

pub fn run(safari: Safari) {
    match safari.subcommand {
        Style::Unwrap { path: _path } => unwrapping::run(&safari.input),
        Style::Thiserror { choose_one } => thiserror::run(&safari.input, choose_one),
        Style::ExternalCrate { alt } => {
            external::run(&safari.input, &alt);
        }
        Style::Anyhow { choice } => {
            anyhow::run(choice);
        }
    }
}

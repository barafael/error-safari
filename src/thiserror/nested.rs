use clap::Parser;
use std::io;
use thiserror::Error;

#[derive(Debug, Parser, Clone, clap::ArgEnum)]
pub enum Operation {
    ThinkAndPray,
    InputOutput,
    Wtf,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Please more thoughts and prayers. At least {thoughts} and {prayers} required")]
    InsufficientThoughtsAndPrayers { thoughts: u8, prayers: u8 },

    #[error("I/O error: {0} occurred somehow")]
    Io(#[source] io::Error),

    #[error("Some other error happened: {0}")]
    WhaHappened(#[from] anyhow::Error),
}

impl From<Operation> for Error {
    fn from(a: Operation) -> Self {
        match a {
            Operation::ThinkAndPray => Self::InsufficientThoughtsAndPrayers {
                thoughts: 5,
                prayers: 34,
            },
            Operation::InputOutput => Self::Io(io::Error::from_raw_os_error(55)),
            Operation::Wtf => Self::WhaHappened(anyhow::anyhow!("Wa happen!?")),
        }
    }
}

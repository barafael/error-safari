use crate::Alternative;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Please more thoughts and prayers. At least {thoughts} and {prayers} required")]
    InsufficientThoughtsAndPrayers { thoughts: u8, prayers: u8 },

    #[error("I/O error: {0} occurred somehow")]
    Io(#[source] io::Error),

    #[error("Some other error happened: {0}")]
    WhaHappened(#[from] anyhow::Error),
}

impl From<Alternative> for Error {
    fn from(a: Alternative) -> Error {
        match a {
            Alternative::A => Error::InsufficientThoughtsAndPrayers {
                thoughts: 5,
                prayers: 34,
            },
            Alternative::B => Error::Io(io::Error::from_raw_os_error(55)),
            Alternative::C => Error::WhaHappened(anyhow::anyhow!("Wa happen!?")),
        }
    }
}

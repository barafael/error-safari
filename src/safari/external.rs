use clap::Parser;
use thiserror::Error;

#[derive(Debug, Parser, Clone, clap::ArgEnum)]
pub enum Alternative {
    DeutscheBahn,
    FamousLastWords,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Some deutsche bahn error: {0}")]
    DeutscheBahn(#[from] deutsche_bahn_delay_reasons::Grund),

    #[error("The AI went apeshit: {0}")]
    FamousLastWord(#[from] famous_last_words::Error),
}

pub fn run(_input: &str, alt: Alternative) {
    if let Err(e) = operation(_input, alt) {
        println!("{}", e);
    }
}

fn operation(_input: &str, alt: Alternative) -> Result<(), Error> {
    match alt {
        Alternative::DeutscheBahn => Err(deutsche_bahn_delay_reasons::get_grund().into()),
        Alternative::FamousLastWords => Err(famous_last_words::get_random_error().into()),
    }
}

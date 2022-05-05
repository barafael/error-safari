use clap::Parser;
use thiserror::Error;

#[derive(Debug, Parser, Clone, clap::ArgEnum)]
pub enum SomeCrate {
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

pub fn run(_input: &str, alt: SomeCrate) {
    if let Err(e) = operation(_input, alt) {
        println!("{}", e);
    }
}

fn operation(_input: &str, alt: SomeCrate) -> Result<(), Error> {
    match alt {
        SomeCrate::DeutscheBahn => Err(deutsche_bahn_delay_reasons::get_grund().into()),
        SomeCrate::FamousLastWords => Err(famous_last_words::get_random_error().into()),
    }
}

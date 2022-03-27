use clap::Parser;

#[derive(Debug, Parser, Clone, clap::ArgEnum)]
pub enum Alternative {
    DeutscheBahn,
    FamousLastWords,
}

pub fn run(_input: String, alt: Alternative) {
    match alt {
        Alternative::DeutscheBahn => {
            println!("{}", deutsche_bahn_delay_reasons::get_grund());
        }
        Alternative::FamousLastWords => {
            println!("{}", famous_last_words::get_random_error());
        }
    }
}

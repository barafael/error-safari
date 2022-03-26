use crate::Alternative;
use std::io;
use thiserror::Error;

mod nested;

#[derive(Debug, Error)]
pub enum Error {
    #[error("It's a failure!")]
    Fail,

    #[error("It's nice, but still a failure! Here's an emoji: {0}")]
    FailNicely(char),

    #[error("Error ocurred in befuddle module: {0}")]
    Befuddle(#[source] nested::Error),
}

pub fn run(input: String, choose_one: Option<Alternative>) {
    let _ = match input.as_str() {
        "fail" => Err(Error::Fail),
        "fail-nice" => Err(Error::FailNicely(random_emoji::generate())),
        _ => Ok(()),
    }
    .map(|_| println!("thiserror operation succeeded!"))
    .map_err(|e| println!("Error during thiserror operation: {e}"));

    if let Some(one) = choose_one {
        println!("{}", nested::Error::from(one));
    }
}

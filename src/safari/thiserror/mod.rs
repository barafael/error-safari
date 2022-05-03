use thiserror::Error;

pub use nested::Operation;

mod nested;

#[derive(Debug, Error)]
pub enum Error {
    #[error("It's a failure!")]
    Fail,

    #[error("It's nice, but still a failure! Here's an emoji: {0}")]
    FailNicely(char),

    #[error("Befuddling in nested module: {0}")]
    FailNested(#[from] nested::Error),
}

pub fn run(input: &str, choose_one: Option<Operation>) {
    match operation(input, choose_one) {
        Ok(()) => println!("thiserror operation succeeded!"),
        Err(e) => println!("Error during thiserror operation: {e}"),
    }
}

fn operation(input: &str, choose_one: Option<Operation>) -> Result<(), Error> {
    match input {
        "fail" => Err(Error::Fail),
        "fail-nice" => Err(Error::FailNicely(random_emoji::generate())),
        "fail-nested" => {
            if let Some(one) = choose_one {
                Err(Error::FailNested(nested::Error::from(one)))
            } else {
                Err(Error::FailNicely(random_emoji::generate()))
            }
        }
        _ => Ok(()),
    }
}

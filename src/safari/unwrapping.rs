pub fn run(input: &str) {
    operation(input).unwrap();
}

fn operation(input: &str) -> Result<(), Error> {
    match input {
        "fail" => Err(Error::Fail),
        "fail-nice" => Err(Error::FailNice),
        _ => Ok(()),
    }
}

#[derive(Debug)]
pub enum Error {
    Fail,
    FailNice,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fail => write!(f, "Operation Failed"),
            Self::FailNice => write!(f, "Sorry, the application failed to successfully succeed"),
        }
    }
}

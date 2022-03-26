use crate::Safari;

mod unwrapping;

impl Safari {
    pub fn run(self) {
        match self.subcommand {
            crate::Subcommand::Unwrap { path } => unwrapping::run(self.input),
            _ => println!("{:?}", self),
        }
    }
}

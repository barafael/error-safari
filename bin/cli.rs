use clap::StructOpt;
use error_safari::{interface::Safari, run};

fn main() {
    let args = Safari::parse();
    run(args);
}

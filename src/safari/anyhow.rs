use anyhow::Context;
use rand::Rng;

pub fn run(choice: u32) {
    let mut rng = rand::thread_rng();
    match operation(choice, rng.gen_range(0..5)) {
        Ok(()) => println!("anyhow operation succeeded!"),
        Err(e) => println!("Error during anyhow operation: {e}"),
    }
}

fn operation(choice: u32, input: u32) -> anyhow::Result<()> {
    match input {
        0 => anyhow::bail!("Bailing out!!!"),
        1 => Err(anyhow::anyhow!("Creating anyhow error in-place")),
        2 => {
            anyhow::ensure!("hello".len() > 10, "String too short");
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Some root cause")).context("With some context tacked on"),
    }
}

use anyhow::Context;
use deutsche_bahn_delay_reasons::Grund;

pub fn run(choice: u32) {
    match operation(choice) {
        Ok(()) => println!("anyhow operation succeeded!"),
        Err(e) => println!("Error during anyhow operation: {e}"),
    }
}

fn operation(choice: u32) -> anyhow::Result<()> {
    match choice {
        0 => anyhow::bail!("Bailing out!!!"),
        1 => Err(anyhow::anyhow!("Creating anyhow error in-place")),
        2 => {
            anyhow::ensure!("hello".len() > 10, "String too short");
            Ok(())
        }
        3 => operation_a().context("operation_a failed!"),
        4 => operation_b().context("operation_b failed!"),
        _ => Err(anyhow::anyhow!("Some root cause"))
            .context("With some context tacked on")
            .context("another"),
    }
}

/// Returns an [`anyhow::Result`]
fn operation_a() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("This function always fails"))
}

/// Returns a custom result from another crate.
fn operation_b() -> Result<(), Grund> {
    Err(deutsche_bahn_delay_reasons::get_grund())
}

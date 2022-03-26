pub fn run(input: String) {
    is_it(&input, "fail").unwrap();
    is_it(&input, "fail-nice").expect("It was fail-nice! I'm offended");
    println!("worked out nicely! Input was: {input:?}");
}

fn is_it(input: &str, it: &str) -> Option<()> {
    if input == it.to_string() {
        None
    } else {
        Some(())
    }
}

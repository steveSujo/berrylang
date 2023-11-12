use std::fs::File;

use parse::parse;

// use std::env;
mod cli;
mod hashtree;
mod parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::handle_commands()?;

    let file = File::open("/home/steve/source/rust/berry/tests/test.txt")?;

    // parse(file, &mut std::io::stdout());

    Ok(())
}

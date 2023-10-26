// use std::env;
mod Hashtree;
mod cli;
mod parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::handle_commands()
}

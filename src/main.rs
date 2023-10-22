// use std::env;
mod cli;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::handle_commands()
}

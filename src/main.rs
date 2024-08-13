use anyhow::Result;
use std::process;

fn main() -> Result<()> {
    let result = tx_calculator::run();

    if let Err(err) = result {
        eprintln!("error running example: {}", err);
        process::exit(1);
    }
    result
}

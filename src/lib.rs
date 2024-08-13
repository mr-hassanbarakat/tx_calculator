use anyhow::Result;
use csv::ReaderBuilder;
use std::env;
use std::fs;
use std::io::BufReader;
mod client;
mod exchange;
mod transaction;

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = fs::File::open(filename.clone())?;
    let reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new().flexible(true).from_reader(reader);
    let mut exchange = exchange::Exchange::new();
    for result in csv_rdr.deserialize() {
        let record: transaction::Tx = result?;
        exchange.process_transaction(record);
    }
    exchange.print_output();
    Ok(())
}

use client::Client;
use std::{collections::HashMap, error::Error, io, process};
use transaction::Tx;

mod client;
mod transaction;

fn main() {
    // let mut input_hash: HashMap<&str, Tx> = HashMap::new();
    // let mut output_hash: HashMap<&str,Client> = HashMap::new();

    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut input_hash: HashMap<u32, Tx> = HashMap::new();
    let mut output_hash: HashMap<u16, Client> = HashMap::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: transaction::Tx = result?;
        match record.tx_type {
            transaction::TxType::Deposit => {
                if let Some(client) = output_hash.get_mut(&record.client) {
                    client.deposit(record.amount);
                } else {
                    output_hash.insert(
                        record.client,
                        client::Client::new(record.client, record.amount),
                    );
                }
            }
            transaction::TxType::Withdrawal => {
                if let Some(client) = output_hash.get_mut(&record.client) {
                    client.withdraw(record.amount);
                }
            }
            transaction::TxType::Dispute => {
                if let (Some(tx), Some(client)) = (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    if !tx.disput {
                        tx.disput = true;
                        client.dispute(tx.amount)
                    }
                }
            }
            transaction::TxType::Resolve => {
                if let (Some(tx), Some(client)) = (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    if tx.disput {
                        tx.disput = true;
                        client.resolve(tx.amount)
                    }
                }
            }
            transaction::TxType::Chargeback => {
                if let (Some(tx), Some(client)) = (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    if !tx.disput {
                        tx.disput = true;
                        client.chargeback(tx.amount)
                    }
                }
            }
        }

        input_hash.insert(record.tx, record);
    }
    println!("################");
    for (key, value) in input_hash.iter() {
        println!("key: {}, value {:?}", key, value);
    }
    println!("################");
    println!("client, available, held, total, locked");
    for (_, client) in output_hash.iter() {
        println!("{}", client);
    }
    println!("################");
    Ok(())
}

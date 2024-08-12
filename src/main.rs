use client::Client;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, hash::Hash, io, process};
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
    // println!("Hello, world!");
    // println!("##############");
    // let mut x = client::Client::new(0, 0.0);
    // println!("{}", x);
    // x.deposit(5.1);
    // println!("deposit 5.1");
    // println!("{}", x);
    // x.dispute(3.0);
    // println!("disput 3.0");
    // println!("{}", x);
    // x.chargeback(2.0);
    // println!("chargeback 2.0");
    // println!("{}", x);
    // x.resolve(1.0);
    // println!("resolve 1.0");
    // println!("{}", x);
    // x.withdraw(2.0);
    // println!("withdraw 2.0");
    // println!("{}", x);
    // println!("################");
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
            transaction::TxType::DESPOSIT => {
                if let Some(client) = output_hash.get_mut(&record.client) {
                    client.deposit(record.amount);
                } else {
                    output_hash.insert(
                        record.client,
                        client::Client::new(record.client, record.amount),
                    );
                }
            }
            transaction::TxType::WITHDRAWAL => {
                if let Some(client) = output_hash.get_mut(&record.client) {
                    client.withdraw(record.amount);
                }
            }
            transaction::TxType::DISPUTE => {
                match (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    (Some(tx), Some(client)) => {
                        if !tx.disput {
                            tx.disput = true;
                            client.dispute(tx.amount)
                        }
                    }
                    (_, _) => {} // do nothing, this is error,
                                 // (None, Some(client)) => {} // do nothing this is error,
                                 // (Some(tx), None) => {} // do nothing is is wrong,
                }
            }
            transaction::TxType::RESOLVE => {
                match (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    (Some(tx), Some(client)) => {
                        if tx.disput {
                            tx.disput = true;
                            client.resolve(tx.amount)
                        }
                    }
                    (_, _) => {} // do nothing, this is error,
                                 // (None, Some(client)) => {} // do nothing this is error,
                                 // (Some(tx), None) => {} // do nothing is is wrong,
                }
            }
            transaction::TxType::CHARGEBACK => {
                match (
                    input_hash.get_mut(&record.tx),
                    output_hash.get_mut(&record.client),
                ) {
                    (Some(tx), Some(client)) => {
                        if !tx.disput {
                            tx.disput = true;
                            client.chargeback(tx.amount)
                        }
                    }
                    (_, _) => {} // do nothing, this is error,
                                 // (None, Some(client)) => {} // do nothing this is error,
                                 // (Some(tx), None) => {} // do nothing is is wrong,
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
    for (_ , client) in output_hash.iter() {
        println!("{}", client);
    }
    println!("################");
    Ok(())
}

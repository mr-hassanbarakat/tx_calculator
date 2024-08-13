use crate::{
    client::{self, Client},
    transaction::{self, TxRecord},
};
use std::collections::HashMap;

pub struct Exchange {
    pub transactions: HashMap<u32, TxRecord>,
    pub clients: HashMap<u16, Client>,
}

impl Exchange {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            clients: HashMap::new(),
        }
    }
    pub fn process_transaction(&mut self, record: transaction::Tx) {
        let tx_id = record.tx;
        let tx_record = TxRecord::from(record);
        match tx_record.tx_type {
            transaction::TxType::Deposit => {
                if let Some(client) = self.clients.get_mut(&tx_record.client) {
                    client.deposit(tx_record.amount);
                } else {
                    self.clients.insert(
                        tx_record.client,
                        client::Client::new(tx_record.client, tx_record.amount),
                    );
                }
                self.transactions.insert(tx_id, tx_record);
            }
            transaction::TxType::Withdrawal => {
                if let Some(client) = self.clients.get_mut(&tx_record.client) {
                    client.withdraw(tx_record.amount);
                    self.transactions.insert(tx_id, tx_record);
                }
            }

            transaction::TxType::Dispute => {
                if let (Some(tx), Some(client)) = (
                    self.transactions.get_mut(&tx_id),
                    self.clients.get_mut(&tx_record.client),
                ) {
                    // When there is a dispute, we set the dispute flag to True
                    tx.dispute = true;
                    client.dispute(tx.amount)
                }
            }

            transaction::TxType::Resolve => {
                if let (Some(tx), Some(client)) = (
                    self.transactions.get_mut(&tx_id),
                    self.clients.get_mut(&tx_record.client),
                ) {
                    // We check the transaction is disputed before we resolve.
                    if tx.dispute {
                        tx.dispute = false;
                        client.resolve(tx_record.amount)
                    }
                }
            }
            transaction::TxType::Chargeback => {
                if let (Some(tx), Some(client)) = (
                    self.transactions.get_mut(&tx_id),
                    self.clients.get_mut(&tx_record.client),
                ) {
                    // We check the transaction is disputed before we chargeback.
                    if tx.dispute {
                        tx.dispute = false;
                        client.chargeback(tx_record.amount)
                    }
                }
            }
        };
    }

    pub fn print_output(&self) {
        println!("client,available,held,total,locked");
        for (_, client) in self.clients.iter() {
            println!("{}", client);
        }
    }
}

#[cfg(test)]
mod tests {
    use transaction::{Tx, TxType};

    use super::*;

    #[test]
    fn test_main_line() {
        let record = Tx {
            tx_type: TxType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.0),
        };
        let mut exchange = Exchange::new();
        exchange.process_transaction(record);
        assert_eq!(exchange.clients.len(), 1);
        assert_eq!(exchange.transactions.len(), 1);
        let record = Tx {
            tx_type: TxType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };

        exchange.process_transaction(record);
        assert_eq!(exchange.clients.len(), 1);
        assert_eq!(exchange.transactions.len(), 1);

        let record = Tx {
            tx_type: TxType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(1.0),
        };
        exchange.process_transaction(record);
        assert_eq!(exchange.clients.len(), 1);
        assert_eq!(exchange.transactions.len(), 2);

        let record = Tx {
            tx_type: TxType::Resolve,
            client: 1,
            tx: 1,
            amount: None,
        };
        exchange.process_transaction(record);
        assert_eq!(exchange.clients.len(), 1);
        assert_eq!(exchange.transactions.len(), 2);

        let record = Tx {
            tx_type: TxType::Chargeback,
            client: 1,
            tx: 1,
            amount: None,
        };
        exchange.process_transaction(record);
        assert_eq!(exchange.clients.len(), 1);
        assert_eq!(exchange.transactions.len(), 2);
    }

    // Test that we change the `dispute` flag correctly.
    #[test]
    fn test_dispute() {
        let record = Tx {
            tx_type: TxType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.0),
        };
        let mut exchange = Exchange::new();
        exchange.process_transaction(record);
        if let Some(tx) = exchange.transactions.get(&1) {
            // dispute is false.
            assert!(!tx.dispute);
        }

        let record = Tx {
            tx_type: TxType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };

        exchange.process_transaction(record);
        if let Some(tx) = exchange.transactions.get(&1) {
            // dispute is true.
            assert!(tx.dispute);
        }

        let record = Tx {
            tx_type: TxType::Resolve,
            client: 1,
            tx: 1,
            amount: None,
        };

        exchange.process_transaction(record);
        if let Some(tx) = exchange.transactions.get(&1) {
            // dispute is false.
            assert!(!tx.dispute);
        }
    }
}

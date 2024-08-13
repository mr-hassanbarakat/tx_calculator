use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TxType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}
impl std::fmt::Display for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TxType::Deposit => write!(f, "Deposit"),
            TxType::Withdrawal => write!(f, "Withdrawal"),
            TxType::Dispute => write!(f, "Dispute"),
            TxType::Resolve => write!(f, "Resolve"),
            TxType::Chargeback => write!(f, "Chargeback"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Tx {
    #[serde(rename = "type")]
    pub tx_type: TxType,
    #[serde(rename = "client")]
    pub client: u16,
    #[serde(rename = "tx")]
    pub tx: u32,
    #[serde(rename = "amount")]
    #[serde(default)]
    pub amount: Option<f32>,
}

impl std::fmt::Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "type: {}, client id: {}, transaction id: {}, amount: {:?}",
            self.tx_type, self.client, self.tx, self.amount
        )
    }
}

pub struct TxRecord {
    pub tx_type: TxType,
    pub client: u16,
    pub amount: f32,
    pub dispute: bool,
}

impl From<Tx> for TxRecord {
    fn from(value: Tx) -> Self {
        Self {
            tx_type: value.tx_type,
            client: value.client,
            amount: if let Some(amount) = value.amount {
                amount
            } else {
                0.0
            },
            dispute: false,
        }
    }
}

impl std::fmt::Display for TxRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "type: {}, client id: {}, amount: {:.4}, is dispute: {}",
            self.tx_type, self.client, self.amount, self.dispute
        )
    }
}

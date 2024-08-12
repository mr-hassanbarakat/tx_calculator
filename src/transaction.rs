use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TxType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "disput")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
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
    pub amount: f32,
    #[serde(skip)]
    pub disput: bool,
}

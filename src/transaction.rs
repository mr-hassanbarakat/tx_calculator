use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TxType {
    #[serde(rename = "deposit")]
    DESPOSIT,
    #[serde(rename = "withdrawal")]
    WITHDRAWAL,
    #[serde(rename = "disput")]
    DISPUTE,
    #[serde(rename = "resolve")]
    RESOLVE,
    #[serde(rename = "chargeback")]
    CHARGEBACK,
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

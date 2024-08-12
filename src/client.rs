use serde::Serialize;

use crate::transaction;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Client {
    pub id: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl Client {
    pub fn new(id: u16, amount: f32) -> Self {
        Client {
            id,
            available: amount,
            held: 0.0,
            total: amount,
            locked: false,
        }
    }
    pub fn deposit(&mut self, amount: f32) {
        self.available += amount;
        self.total += amount
    }
    pub fn withdraw(&mut self, amount: f32) {
        if self.available - amount > 0.0000 {
            self.available = self.available - amount;
            self.total = self.total - amount;
        }
    }

    pub fn dispute(&mut self, amount: f32) {
        self.available -= amount;
        self.held += amount;
    }
    pub fn resolve(&mut self, amount: f32) {
        self.available += amount;
        self.held -= amount;
    }
    pub fn chargeback(&mut self, amount: f32) {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, {:.4}, {:.4}, {:.4}, {}",
            self.id, self.available, self.held, self.total, self.locked
        )
    }
}

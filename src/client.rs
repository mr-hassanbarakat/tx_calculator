use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Client {
    #[serde(rename = "client")]
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
    // For a deposit, the available and total should increase by the amount.
    pub fn deposit(&mut self, amount: f32) {
        self.available += amount;
        self.total += amount;
    }
    // For withdrawal, the available, and total should decrease by the amount
    // after we check that we have enough funds.
    pub fn withdraw(&mut self, amount: f32) {
        if self.available - amount > 0.0000 {
            self.available -= amount;
            self.total -= amount;
        }
    }

    // In a dispute, the amount disputed should move from available to held.
    // total funds should not change.
    pub fn dispute(&mut self, amount: f32) {
        self.available -= amount;
        self.held += amount;
    }
    // When dispute is resolved. Amount held should move to available.
    // Total is not changed.
    pub fn resolve(&mut self, amount: f32) {
        self.available += amount;
        self.held -= amount;
    }
    // In a chargeback, Amount held will be withdrawn, and so we reduce the amount from held and total.
    // the client is locked
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
            "{},{:.4},{:.4},{:.4},{}",
            self.id, self.available, self.held, self.total, self.locked
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_successful() {
        let mut client = Client::new(1, 0.0);
        client.deposit(2.0);
        let expected_client = Client {
            id: 1,
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: false,
        };
        assert_eq!(client, expected_client);
    }

    #[test]
    fn test_dispute_resolve_successful() {
        let mut client = Client::new(1, 0.0);
        client.deposit(2.0);

        client.dispute(2.0);
        let mut expected_client = Client {
            id: 1,
            available: 0.0,
            held: 2.0,
            total: 2.0,
            locked: false,
        };
        assert_eq!(client, expected_client);

        client.resolve(2.0);

        expected_client.held -= 2.0;
        expected_client.available += 2.0;
        assert_eq!(client, expected_client);
    }

    #[test]
    fn test_chargeback_successful() {
        let mut client = Client::new(1, 0.0);
        client.deposit(2.0);
        client.dispute(2.0);
        client.chargeback(2.0);
        let expected_client = Client {
            id: 1,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: true,
        };
        assert_eq!(client, expected_client);
    }

    #[test]
    fn test_withdraw_successful() {
        let mut client = Client::new(1, 0.0);
        client.deposit(2.0);

        client.withdraw(1.0);
        let expected_client = Client {
            id: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
        };
        assert_eq!(client, expected_client);

        // Cannot withdraw more than we have.
        client.withdraw(5.0);
        assert_eq!(client, expected_client);
    }
}

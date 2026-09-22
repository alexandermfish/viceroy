use crate::templar::Templar;
use crate::currency::Currency;

pub struct Account<T: Currency> {
    name: String,
    balance: T,
}

impl<T: Currency> Account<T> {
    pub fn new(name: String) -> Account<T> {
        return Account {
            name,
            balance: Currency::zero(),
        };
    }
    pub fn statement(&self) -> String {
        return format!("{}: {}", self.name, self.balance.amount());
    }
    pub fn add(&mut self, amount: u64 ){
        self.balance.
    }
}


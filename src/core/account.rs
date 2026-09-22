use crate::core::user::User;
use crate::money::currency::Currency;


pub struct Account<T: Currency> {
    name: String,
    balance: T,
    id: u64,
    user_id: u64
}

impl<T: Currency> Account<T> {
    pub fn new(name: String, user: &User) -> Account<T> {
        return Account {
            name,
            balance: Currency::zero(),
            id: 0,
            user_id:user.id()
        };
    }
    pub fn statement(&self) -> String {
        return format!("{}: {}", self.name, self.balance.amount());
    }
    pub fn add(&mut self, amount: u64 ){
        self.balance.add(amount);
    }
}


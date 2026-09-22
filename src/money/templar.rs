use crate::money::currency::Currency;

pub struct Templar {
    amount: u64,
}
// "TEMPoraL dollAR" : "Templar".
impl Currency for Templar {
    fn new(amount: u64) -> Templar {
        Templar {
            amount
        }
    }
    fn amount(&self) -> u64 {
        self.amount
    }
    fn add(&mut self, amount: u64) {
        self.amount = self.amount + amount;
    }
}


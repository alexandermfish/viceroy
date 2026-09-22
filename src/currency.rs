pub trait Currency :Sized {
    fn new(value: u64) -> Self;
    fn amount(&self) -> u64;

    fn add(&mut self, amount: u64);

    fn zero() -> Self {
        Self::new(0)
    }
}
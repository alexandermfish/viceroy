mod account;
mod templar;
mod currency;

use crate::account::Account;
use crate::templar::Templar;
fn main() {
    let mut alex_temporal_account = Account::<Templar>::new(String::from("Alex"));
    let tina_temporal_account = Account::<Templar>::new(String::from("Tina"));
    alex_temporal_account.add(1500);
    println!("{}", alex_temporal_account.statement());
    println!("{}", tina_temporal_account.statement());
}
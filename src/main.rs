mod account;
mod templar;
mod currency;

use crate::account::Account;
use crate::templar::Templar;
fn main() {
    let alex_temporal_account = Account::<Templar>::new(String::from("Alex"));
    let tina_temporal_account = Account::<Templar>::new(String::from("Tina"));
    alex_temporal_account.add
    println!("{}", alex_temporal_account.statement());
    println!("{}", tina_temporal_account.statement());
}
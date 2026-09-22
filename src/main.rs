
mod core;
mod money;

use crate::core::account::Account;
use crate::core::user::User;
use crate::money::templar::Templar;

fn main() {

    let mut alex_temporal_account =
        Account::<Templar>::new(String::from("Alex's Templar Account"), &User::new("alex".to_string(), 0));
    let tina_temporal_account =
        Account::<Templar>::new(String::from("Tina's Templar Account"), &User::new("tina".to_string(), 0));
    alex_temporal_account.add(1500);
    println!("{}", alex_temporal_account.statement());
    println!("{}", tina_temporal_account.statement());
}
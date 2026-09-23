use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LedgerEvent {
    CreateUser {
        id: u64,
        name: String,
    },
    Transfer {
        from: u64,
        to: u64,
        amount: u64,
    },
}
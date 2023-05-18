
enum Action {
    /// This action determines the amount to be deposited
    /// from the account and the time it took for the person to withdraw from the account
    Deposit {
        /// Amount in float
        amount: f64,
        /// Time in milliseconds
        duration: usize
    },
    /// This action determines the amount to be withdrawn from the account
    /// and the time it took for the person to withdraw from the account
    Withdrawal{
        /// Amount to be withdrawn
        amount: f64,
        duration: usize
    },
    ForeignExchange{
        amount: f64,
        initial_currency: String,
        new_currency: String,
        duration: usize
    },
    Payment {
        amount: f64,
        service: String,
        duration: usize
    }
}

pub struct Teller {
    teller_id: String,
    current_action: Action,
}

pub struct Customer {
    account_number: i32,
    waiting_time: usize,
    event: Action,
    position: u8
}
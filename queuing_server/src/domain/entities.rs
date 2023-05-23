use actix_web::http::header::q;

pub enum Action {
    /// This action determines the amount to be deposited
    /// from the account and the time it took for the person to withdraw from the account
    Deposit {
        /// Amount in float
        amount: f64,
        /// Time in milliseconds
        service_time: usize
    },
    /// This action determines the amount to be withdrawn from the account
    /// and the time it took for the person to withdraw from the account
    Withdrawal{
        /// Amount to be withdrawn
        amount: f64,
        service_time: usize
    },
    ForeignExchange{
        amount: f64,
        initial_currency: String,
        new_currency: String,
        service_time: usize
    },
    Payment {
        amount: f64,
        service: String,
        service_time: usize
    }
}

pub struct Teller {
    teller_id: String,
    current_action: Action,
}

impl Teller {
    fn new(teller_id: String, action: Action, average_time: usize, best_time: usize) -> Self {
        Self {
            teller_id,
            current_action: action
        }
    }
}



pub struct Customer {
    account_number: String,
    waiting_time: usize,
    event: Action,
    position: u8,
    queue_number: u8,
}

impl Customer {
    fn new(queue_number: u8, position: u8, event: Action, waiting_time: usize, account_number: String) -> Customer {
        Customer {
            account_number,
            queue_number,
            position,
            event,
            waiting_time,
        }
    }
    fn change_teller(mut self, queue_number: u8, position: u8, waiting_time: usize) -> Self {
        self.queue_number = queue_number;
        self.position = position;
        self.waiting_time = waiting_time;
        self
    }
}
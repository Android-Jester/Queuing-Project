
enum Actions {
    WithDrawal,
    Deposit,
    ForeignExchange,
    Enquiry
}


pub struct Teller {
    pub name: String,
    pub action: String,
    pub average_time: usize,
    pub best_time: usize,
}

pub struct User {
    pub name: String,
    pub action: String,
}

impl Teller {
    fn new(name: String, action: String, average_time: usize, best_time: usize) -> Self {
        Self {
            name,
            average_time,
            best_time,
            action
        }
    }
 
}

fn best_line(tellers: Vec<Teller>, action: String) {
    
}

fn change_teller() {}

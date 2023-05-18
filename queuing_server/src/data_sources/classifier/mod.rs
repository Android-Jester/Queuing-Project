use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};

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
    let mut random_class = RandomForestClassifierOptions::new()
        .seed(10);
    random_class.

}

fn change_teller() {}

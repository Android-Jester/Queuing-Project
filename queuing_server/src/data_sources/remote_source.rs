use std::char::MAX;
use std::num::{NonZeroIsize, NonZeroUsize};

use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use randomforest::criterion::{Mse, Gini};
use randomforest::table::{Table, TableBuilder};
use crate::domain::entities::{Teller, Action};
impl Teller {
    fn new(teller_id: String, action: Action, average_time: usize, best_time: usize) -> Self {
        Self {
            teller_id,
            current_action: action
        }
    }
    fn calc_service_time(&self) -> f64 {
            let service_time = match teller.current_action {
                Action::Deposit { amount, duration } => duration,
                Action::Withdrawal { amount, duration } => duration,
                Action::ForeignExchange {
                    amount, initial_currency, new_currency, duration
                } => duration,
                Action::Payment { amount, duration, service  } => duration
            };
        service_time.into()
        }
    }

fn tellers_table(tellers: Vec<f64>) {
    let mut table_builder = TableBuilder::new();
    for teller in tellers {
        // table_builder.add_row(teller.into(), ).expect("No Data");
    }
}


fn best_line(tellers: Table, features: &[f64], action: String) -> f64 {
    const MAX_FEATURES: NonZeroUsize = NonZeroUsize::new(12).ok_or(0).unwrap();
    const MAX_SAMPLES: NonZeroUsize = NonZeroUsize::new(12).ok_or(0).unwrap();
    let mut random_class: RandomForestClassifier = RandomForestClassifierOptions::new()
        .seed(10)
        .max_features(MAX_FEATURES)
        .max_samples(MAX_SAMPLES)
        .fit(Gini, tellers);
    let pred = random_class.predict(features);
    pred
}

fn change_teller() {}

use std::char::MAX;
use std::num::{NonZeroIsize, NonZeroUsize};

use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use randomforest::criterion::{Mse, Gini};
use randomforest::table::Table;
use crate::domain::entities::{Teller, Action};
impl Teller {
    fn new(teller_id: String, action: Action, average_time: usize, best_time: usize) -> Self {
        Self {
            teller_id,
            current_action: action
        }
    }
 
}

fn best_line(tellers: Table, features: &[f64], action: String) -> f64 {
    let MAX_FEATURES = NonZeroUsize::new(12).ok_or(0).unwrap();
    let MAX_SAMPLES = NonZeroUsize::new(12).ok_or(0).unwrap();
    let mut random_class: RandomForestClassifier = RandomForestClassifierOptions::new()
        .seed(10)
        .max_features(MAX_FEATURES)
        .max_samples(MAX_SAMPLES)
        .fit(Gini, tellers);
    let pred = random_class.predict(features);
    pred
}

fn change_teller() {}

use crate::data::prelude::*;
use crate::prelude::*;
use randomforest::criterion::Gini;
use randomforest::table::TableBuilder;
use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use std::num::NonZeroUsize;

///Split data into test data and train data to verify the fact
fn classify_data(complete_data: &(Vec<[f64; 4]>, Vec<u8>)) -> (Vec<[f64; 4]>, Vec<u8>) {
    let (data, targets) = complete_data;
    (data.to_owned(), targets.to_owned())
}

/// Training the model
fn train_model(
    max_features: usize,
    max_samples: usize,
    trees: u64,
    train_data: (Vec<[f64; 4]>, Vec<u8>),
) -> RandomForestClassifier {
    let features = NonZeroUsize::new(max_features).unwrap();
    let samples = NonZeroUsize::new(max_samples).unwrap();
    let mut binding = RandomForestClassifierOptions::new();
    let random_forest_option_data = binding
        .seed(trees)
        .max_features(features)
        .max_samples(samples);

    let mut table_builder = TableBuilder::new();
    let (data, target) = train_data;
    for (item_count, info) in data.iter().enumerate() {
        table_builder
            .add_row(&info[0..], target[item_count] as f64)
            .expect("Data cannot be added");
    }
    let table = table_builder.build().expect("No Table built");
    random_forest_option_data.fit(Gini, table)
}

/// predict best line
pub fn prediction(pred: [f64; SERVER_COUNT]) -> u8 {
    let service_times_data = get_all_service_times();
    let data = classify_data(&service_times_data);
    let classifier = train_model(100, 10, 20, data);
    let acc_pred = classifier.predict(&pred);
    acc_pred as u8
}

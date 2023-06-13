use randomforest::criterion::Gini;
use randomforest::table::TableBuilder;
use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use std::num::NonZeroUsize;


///Split data into test data and train data to verify the fact
fn classify_data(
    complete_data: &Vec<(Vec<f64>, f64)>,
    ratio: usize,
) -> (Vec<(Vec<f64>, f64)>, Vec<(Vec<f64>, f64)>) {
    let total_len = complete_data.len();
    let range_limit = total_len / ratio;
    let train = complete_data[..range_limit].to_vec();
    let test = complete_data[range_limit..].to_vec();
    (train, test)
}



/// Training the model
fn train_model(
    max_features: usize,
    max_samples: usize,
    trees: u64,
    service_time_train_data: Vec<(Vec<f64>, f64)>,
) -> RandomForestClassifier {
    let features = NonZeroUsize::new(max_features).unwrap();
    let samples = NonZeroUsize::new(max_samples).unwrap();
    let mut binding = RandomForestClassifierOptions::new();
    let random_forest_option_data = binding
        .seed(trees)
        .max_features(features)
        .max_samples(samples);

    let mut table_builder = TableBuilder::new();
    for (pos, data) in service_time_train_data.iter().enumerate() {
        let (service_data, service_target) = data;
        table_builder
            .add_row(service_data, *service_target as f64)
            .expect("Data cannot be added");
    }
    let table = table_builder.build().expect("No Table built");
    random_forest_option_data.fit(Gini, table)
}

/// Test the model for accuracy
fn test_model_prediction(
    mut randomforest: &RandomForestClassifier,
    service_time_test_data: &Vec<f64>,
) -> f64 {
    let prediction = randomforest.predict(service_time_test_data);
    prediction
}

// /// predict best line
// pub fn prediction() {
//     //     // TODO: Obtain Data from database
//
//     //     // TODO: Classify Data into target and features
//     let (train, test) = classify_data(&transaction_data, 80);
//     //     // TODO: Train Model
//     let mut classifier = train_model(100, 10, 20, train);
//     // let acc_pred = test_model_prediction(&mut classifier, test);
//     //     // TODO: Predict Result
// }
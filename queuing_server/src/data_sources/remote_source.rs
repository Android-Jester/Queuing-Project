use std::char::MAX;
use std::fmt::format;
use std::num::{NonZeroIsize, NonZeroUsize};

use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use randomforest::criterion::{Mse, Gini};
use randomforest::table::{Table, TableBuilder, TableError};
use crate::domain::entities::{Teller, Action, Customer};

fn train_model(model :&mut RandomForestClassifierOptions, service_time_table: Table) -> RandomForestClassifier {
    model.fit(Gini, service_time_table)
}

fn start_model<'a>( max_features :NonZeroUsize, max_samples: NonZeroUsize) -> &'a mut RandomForestClassifierOptions {
    RandomForestClassifierOptions::new()
        .seed(10)
        .max_features(max_features)
        .max_samples(max_samples)
}

fn classify_data(service_time_table_builder: &mut TableBuilder, service_time_features: Vec<f64>, service_time_target: f64) -> std::io::Result<()> {
    if let Ok(data ) = service_time_table_builder.add_row(service_time_features.into(), service_time_target) {
        Ok(())
    } else {
        match TableError {
            //TODO: Handling Errors on Data acquired from database
            TableError::EmptyTable => {},
            TableError::NonFiniteTarget => {},
            TableError::ColumnSizeMismatch => {}
        }
    }
}

fn predict() {
    // TODO: Obtain Data from database
    // TODO: Classify Data into target and features
    // classify_data()
    // TODO: Train Model
    // train_model()
    // TODO: Predict Result
}



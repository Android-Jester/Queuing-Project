use std::char::MAX;
use std::env;
use std::fmt::format;
use std::num::{NonZeroIsize, NonZeroUsize};
use actix_web::web::Data;

use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
use randomforest::criterion::{Mse, Gini};
use randomforest::table::{Table, TableBuilder, TableError};

use dotenvy::dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

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

fn classify_data(service_time_table_builder: &mut TableBuilder, service_time_features: &Vec<f64>, service_time_target: f64) -> Result<(), TableError> {
     service_time_table_builder.add_row(service_time_features.as_slice(), service_time_target)
}

fn predict() {
    // TODO: Obtain Data from database
    // TODO: Classify Data into target and features
    // classify_data()
    // TODO: Train Model
    // train_model()
    // TODO: Predict Result
}

fn establish_db_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match MysqlConnection::establish(&database_url) {
        Ok(connection) => connection,
        Err(_) => panic!("Error Connecting to {}", database_url)
    }
}

fn run_db() -> Vec<Teller> {
    let connection = &mut establish_db_connection();
    //TODO: Repl0ace Data with actual database model value
    data
        .filter( || {})
        .load::<Data>(connection)
        .expect("Error Loading Queue Data")
}

fn add_teller(connection: &mut MysqlConnection, teller: &Teller) {
    //TODO: Replace Data with actual database model value
    diesel::insert_into(data::table)
        .values(teller)
        .get_result(conn)
        .expect("Error Adding Teller")
}


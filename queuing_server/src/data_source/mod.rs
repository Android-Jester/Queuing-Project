use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;
use std::env;

/// get connection to the database
pub fn establish_db() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect("Unable to connect to DB")
}

///Calculate avg of service_times
pub fn calc_best_avg(avg_times: &[f64]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best).unwrap();
    pos as u8
}

/// Calculate the average arrival time
pub fn avg_arrival_time() {}

pub mod random_forest_classification {
    use randomforest::criterion::Gini;
    use randomforest::table::TableBuilder;
    use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
    use std::num::NonZeroUsize;

    use super::db_actions::get_all_service_times;
    use super::establish_db;

    ///Split data into test data and train data to verify the fact
    fn classify_data(
        complete_data: &Vec<(Vec<f32>, f32)>,
        ratio: usize,
    ) -> (Vec<(Vec<f32>, f32)>, Vec<(Vec<f32>, f32)>) {
        let total_len = complete_data.len();
        let range_limit = total_len / ratio;
        let train = complete_data[..range_limit].to_vec();
        let test = complete_data[range_limit..].to_vec();
        (train, test)
    }

    /// Initializes the RandomForestClassfifier
    fn start_model<'a>(
        max_features: usize,
        max_samples: usize,
        trees: u64,
    ) -> RandomForestClassifierOptions {
        let features = NonZeroUsize::new(max_features).unwrap();
        let samples = NonZeroUsize::new(max_samples).unwrap();
        let random_forest_option_data = RandomForestClassifierOptions::new()
            .seed(trees)
            .max_features(features)
            .max_samples(samples);
        random_forest_option_data.to_owned()
    }

    /// Training the model
    fn train_model(
        model: &mut RandomForestClassifierOptions,
        service_time_train_data: Vec<(Vec<f32>, f32)>,
    ) -> RandomForestClassifier {
        let mut table_builder = TableBuilder::new();
        for (pos, data) in service_time_train_data.iter().enumerate() {
            let (service_data, service_target) = data;
            table_builder
                .add_row(service_data, *service_target)
                .expect("Data cannot be added");
        }
        let table = table_builder.build().expect("No Table built");
        model.fit(Gini, table)
    }

    /// Test the model for accuracy
    fn test_model_prediction(
        mut randomforest: &RandomForestClassifier,
        service_time_test_data: &Vec<f64>,
    ) -> f64 {
        let prediction = randomforest.predict(service_time_test_data);
        prediction
    }

    /// predict best line
    pub fn prediction() {
        //     // TODO: Obtain Data from database
        let transaction_data = get_all_service_times(&mut establish_db());
        //     // TODO: Classify Data into target and features
        let (train, test) = classify_data(&transaction_data, 80);
        //     // TODO: Train Model
        let mut generative = start_model(10, 100, 50);
        let mut classifier = train_model(&mut generative, train);
        let acc_pred = test_model_prediction(&mut classifier, test.0);
        //     // TODO: Predict Result
    }
}

pub mod db_actions {
    use crate::data::schema::transaction::dsl::*;
    use crate::data::{models::Transaction, schema::transaction};
    use diesel::prelude::*;
    use diesel::{Connection, MysqlConnection};

    fn add_transaction(conn: &mut MysqlConnection, customer: Transaction) -> bool {
        let insert_transaction = conn.transaction(|conn| {
            diesel::insert_into(transaction::table)
                .values(customer)
                .execute(conn)
        });
        match insert_transaction {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub fn get_all_service_times(conn: &mut MysqlConnection) -> Vec<(Vec<f32>, f32)> {
        let mut data: Vec<f32> = Vec::new();
        let extract_service = conn
            .transaction(|connection| {
                let results = transaction
                    .select(Transaction::as_select())
                    .load(connection);
                results
            })
            .expect("Unknown Values");
    }
}

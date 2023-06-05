use std::{env};
use diesel::{Connection, MysqlConnection};
use dotenvy::dotenv;


/// get connection to the database
pub fn establish_db() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect("Unable to connect to DB")
}

/// list all data in the service table

// TODO: List all service_data from db
// pub fn list_service_table() -> Vec<(Vec<f64>, f64)> {
//
// }




///Calculate avg of service_times
pub fn calc_best_avg(avg_times: &[f64]) -> u8 {
    let mut best= avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data | *data == best ).unwrap();
    pos as u8
}



pub mod random_forest_classification {
    use randomforest::{RandomForestClassifier, RandomForestClassifierOptions};
    use randomforest::criterion::Gini;
    use randomforest::table::TableBuilder;
    use std::num::NonZeroUsize;

    ///Split data into test data and train data to verify the fact
    fn classify_data(complete_data: &Vec<(Vec<f64>, f64)>, ratio: usize) -> (Box<&[(Vec<f64>, f64)]>, Box<&[(Vec<f64>, f64)]>) {
        let total_len = complete_data.len();
        let range_limit = total_len / ratio;
        let data = complete_data.chunks(range_limit);
        let train = Box::new(data[0]);
        let test = Box::new(data[1]);
        (train, test)
    }

    /// Initializes the RandomForestClassfifier
    fn start_model<'a>(
        max_features: usize,
        max_samples: usize,
        trees: u64
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
        service_time_train_data: Vec<(&[f64], f64)>,
    ) -> RandomForestClassifier {
        let mut table_builder = TableBuilder::new();
        for (pos, data) in service_time_train_data.iter().enumerate() {
            let (service_data, service_target) = data;
            table_builder.add_row(service_data, *service_target).expect("Data cannot be added");
        }
        let table = table_builder.build().expect("No Table built");
        model.fit(Gini, table)
    }

    /// Test the model for accuracy
    fn test_model_prediction(
        mut randomforest: &RandomForestClassifier,
        service_time_test_data: &Vec<f64>
    ) -> f64 {
        let prediction = randomforest.predict_individuals(service_time_test_data);
        prediction
    }


    // predict best line
    // pub fn prediction() -> u8 {
    //     // TODO: Obtain Data from database
    //     // let (service_data, service_targets) = list_service_table();
    //     // TODO: Classify Data into target and features
    //     // let (train, test) = classify_data(service_data, 80);
    //     // TODO: Train Model
    //     // let mut generative = start_model(10, 100);
    //     // let mut classifier = train_model(&mut generative, train);
    //     // let acc_pred = test_model_prediction(&mut classifier, test.0);
    //     // TODO: Predict Result
    // }

}

pub mod db_actions {
    use diesel::{Connection, MysqlConnection, QueryDsl, RunQueryDsl};
    use crate::data::models::{Transactions, User};
    use crate::data::schema::Users;
    use diesel::prelude::*;
    use crate::data::schema::Users::transaction;

    fn add_customer(
        conn: &mut MysqlConnection,
        customer: User
    ) -> bool {
        let insert_customer = conn.transaction(|conn| {
            diesel::insert_into(Users::table)
                .values(customer)
                .execute(conn)
        });
        match insert_customer {
            Ok(_) => true,
            Err(_) => false
        }
    }
    fn get_all_service_times(conn: &mut MysqlConnection,) -> Vec<(Vec<f64>, f64)> {
        use crate::data::schema::Users::dsl::*;
        let extract_service = conn.transaction(|connection | {
            let results = Users
                .select(User::as_select())
                .load(connection)
                .expect("Error loading data");
            Ok(())
        } );
    }
    fn insert_service_data(conn: &mut MysqlConnection, transaction: /*TODO: FIX TRANSACTIONS*/Transactions) {
        let insert_customer = conn.transaction(|conn| {
            diesel::insert_into(Users::table)
                .values(transaction)
                .execute(conn)
        });
    }
}
use std::error::Error;
use image::io::Reader;
use crate::domain::entities::Teller;

pub trait QueuingRepository {
    fn train_model<R>(data: Reader<R>) -> Result<String, dyn Error>;
    fn obtain_predictions() -> [f64];
    fn get_best_queue() -> String;
}

trait Calculations {
    fn calculate_service_time(teller: Teller);
    fn calculate_average_service_time(teller: [Teller; MAX_NUMBER_OF_SERVERS]);
    fn calculate_average_number_customers(customers: Vec<Customers>);
}
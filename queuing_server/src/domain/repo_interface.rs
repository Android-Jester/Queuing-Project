use csv::Reader;
use crate::domain::entities::Teller;
use crate::core::constants::MAX_NUMBER_OF_SERVERS;
use super::entities::Customer;

pub trait QueuingRepository {
    fn train_model<R>(data: Reader<R>) -> std::io::Result<String>;
    // fn obtain_predictions() -> [f64];
    fn get_best_queue() -> String;
    fn signup(&self) -> String;
    fn login(&self) -> String;

}

trait Calculations {
    fn calculate_service_time(teller: Teller, customer: Customer);
    fn calculate_average_service_time(teller: [Teller; MAX_NUMBER_OF_SERVERS]);
    fn calculate_average_number_customers(customers: Vec<Customer>);
}
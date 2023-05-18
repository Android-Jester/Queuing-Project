use std::error::Error;
use image::io::Reader;
use crate::data_sources::classifier::Teller;
use crate::domain::repo_interface::QueuingRepository;
impl<T, Q> QueuingRepository for Teller {
    fn train_model<R>(data: Reader<R>) -> Result<String, dyn Error> {
        todo!("Code for training the model from the classifier lib")
    }

    fn obtain_predictions() -> Box<[f64]> {
        todo!("Take the result of the predictions and display to client")
    }

    fn get_best_queue() -> String {
        todo!("Obtain the best queue")
    }
}
use crate::domain::{repo_interface::QueuingRepository, entities::Teller};
impl QueuingRepository for Teller {
    fn train_model<R>(data: csv::Reader<R>) -> std::io::Result<String> {
        todo!("Code for training the model from the classifier lib")
    }

    fn get_best_queue() -> String {
        todo!("Obtain the best queue")
    }

    fn signup(&self) -> String {
        todo!()
    }

    fn login(&self) -> String {
        todo!()
    }
}
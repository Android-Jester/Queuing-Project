pub mod data_acquision;

pub struct Analytics {
    average_service_time: f64,
    average_customer_count: f64,
    average_waiting_system: f64,
    average_waiting_queue: f64,
}

pub mod prelude {
    pub use super::data_acquision::*;
    use super::Analytics;
    // pub fn get_analytics() -> Result<Analytics, String> {
    //     let arrival_rate = 0;
    //     let analytics = Analytics {
    //         average_customer_count: average_customer_count(arrival_rate, c, mu)
    //     }

    //     Ok(analytics)
    // }
}

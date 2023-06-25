// fn calc_avg_time(service_times: Vec<f64>) -> f64 {
//     let mut sum = 0.0;
//     for time in &service_times {
//         sum += time;
//     }
//     sum / service_times.len() as f64
// }
//
// fn probability_of_none(arrival_rate: f64, time_interval: f64) -> f64 {
//     E.powf(-(arrival_rate * time_interval))
// }
//
// fn factorial(num: u8) -> u64 {
//     let mut mul: u64 = 1;
//     for i in 2..num {
//         mul = mul * i as u64;
//     }
//     mul
// }
//
// pub fn efficiency_rate(arrival_rate: f64, service_times: f64) -> f64 {
//     arrival_rate / (SERVER_COUNT as f64 * service_times)
// }
//
// /// Average number of customers in queue + currently in service
// pub fn average_customer_count_system(
//     arrival_rate: f64,
//     service_rate: f64,
//     service_times: Vec<f64>,
// ) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let avg_service = calc_avg_time(service_times);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let expo = efficiency_rate.powi(SERVER_COUNT as i32);
//     let div = avg_service * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     ((expo / div) * probability_of_none(arrival_rate, 1.0) + (1.0 / avg_service)) * arrival_rate
// }

// Average number of customers in queue only
// pub fn average_customer_count_queue(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let expo =
//         efficiency_rate.powi(SERVER_COUNT as i32 + 1) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     expo / div
// }

// pub fn average_customer_waiting_time_queue(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     top / div
// }

// pub fn average_customer_waiting_time_system(arrival_rate: f64, service_rate: f64) -> f64 {
//     let efficiency_rate = efficiency_rate(arrival_rate, service_rate);
//     let top = efficiency_rate.powi(SERVER_COUNT as i32) * probability_of_none(arrival_rate, 1.0);
//     let frac = (1.0 - (efficiency_rate / SERVER_COUNT as f64)).powi(2);
//     let div = service_rate * factorial(SERVER_COUNT as u8) as f64 * SERVER_COUNT as f64 * frac;
//     top / div + (1.0 / service_rate)
// }

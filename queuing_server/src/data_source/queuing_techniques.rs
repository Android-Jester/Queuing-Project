use std::f64::consts::E;


///Calculate best of service_times
pub fn calc_best_avg(avg_times: &[f64]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best).unwrap();
    pos as u8
}


fn calc_avg_time(service_times: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for time in &service_times {
        sum = sum + time;
    }
    let res = sum / service_times.len() as f64;
    res
}

fn probability_of_none(arrival_rate: f64, time_interval: f64) -> f64 {
    E.powf(-(arrival_rate * time_interval))
}

fn factorial(num: u8) -> u64 {
    let mut mul: u64 = 1;
    for i in 2..num {
        mul = mul * i as u64;
    }
    mul
}

pub fn average_customer_count_system(
    efficiency_rate: f64,
    server_count: u8,
    arrival_rate: f64,
    service_times: Vec<f64>,
) -> f64 {
    let avg_service = calc_avg_time(service_times);
    let frac = (1.0 - (efficiency_rate / server_count as f64)).powi(2);
    let expo = efficiency_rate.powi(server_count.into());
    let div = avg_service * factorial(server_count.into()) as f64 * server_count as f64 * frac;
    let res = ((expo / div) * probability_of_none(arrival_rate, 1.0) + (1.0 / avg_service))
        * arrival_rate;
    res
}

pub fn average_customer_count_queue(
    efficiency_rate: f64,
    server_count: u8,
    arrival_rate: f64,
) -> f64 {
    let expo = efficiency_rate.powi(server_count as i32 + 1);
    let frac = (1.0 - (efficiency_rate / server_count as f64)).powi(2);
    let div = factorial(server_count) as f64 * server_count as f64 * frac;
    let res = (expo / div) * probability_of_none(arrival_rate, 1.0);
    res
}
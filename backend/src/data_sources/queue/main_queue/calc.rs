use crate::prelude::*;

fn calc_best_avg(avg_times: [f64; SERVER_COUNT]) -> u8 {
    let mut best = avg_times[0];
    for i in avg_times.iter() {
        best = best.max(*i)
    }
    let pos = avg_times.iter().position(|data| *data == best);
    pos.unwrap() as u8
}

pub fn get_all_service_times() -> (Vec<[f64; 4]>, Vec<u8>) {
    let transactions = list_transactions().unwrap();
    let mut server_1: Vec<f64> = Vec::new();
    let mut server_2: Vec<f64> = Vec::new();
    let mut server_3: Vec<f64> = Vec::new();
    let mut server_4: Vec<f64> = Vec::new();
    let mut service_times: Vec<[f64; SERVER_COUNT]> = Vec::new();
    let mut best_queue: Vec<u8> = Vec::new();
    for transaction in transactions {
        if let Ok(teller) = find_teller(transaction.server_id) {
            match teller.station {
                1 => server_1.push(transaction.duration as f64),
                2 => server_2.push(transaction.duration as f64),
                3 => server_3.push(transaction.duration as f64),
                4 => server_4.push(transaction.duration as f64),
                _ => {}
            }
        }
    }
    info!("Server 1: {:?}", server_1);
    info!("Server 2: {:?}", server_2);
    info!("Server 3: {:?}", server_3);
    info!("Server 4: {:?}", server_4);

    for data in 0..server_1.len() {
        service_times.push([
            server_1[data],
            server_2[data],
            server_3[data],
            server_4[data],
        ])
    }

    for ser_time in &service_times {
        best_queue.push(calc_best_avg(*ser_time))
    }
    (service_times, best_queue)
}

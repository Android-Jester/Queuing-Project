use crate::prelude::*;
#[test]
fn test_query() {
    let service_time = average_service_time();
    println!("Service Time(Avg): {:?}", service_time.unwrap())
}

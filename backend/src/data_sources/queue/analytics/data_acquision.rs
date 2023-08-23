fn performance_coefficient(lambda: f64, c: f64, mu: f64) -> f64 {
        lambda / (c * mu)
}


fn probability_of_zero(lambda: f64, c: usize, mu: f64) -> f64 {
    let rho = performance_coefficient(lambda,  c as f64, mu);
    let sum = (0..=c).fold(0.0, |acc, n| acc + rho_pow(rho, n) / factorial(n));
    let last_term = rho_pow(rho, c + 1) / factorial(c) * (1.0 - rho);
    let denominator = 1.0 / (sum + last_term);
    denominator.powf(-1.0)
}

// Helper function to calculate ρ^n
fn rho_pow(ρ: f64, n: usize) -> f64 {
    ρ.powi(n as i32)
}

// Helper function to calculate factorial of a number
fn factorial(n: usize) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * x as f64)
}
pub fn get_service_times() {
    
}
pub fn average_waiting_times() {}
pub fn average_customer_count(lambda: f64, c: usize, mu: f64) -> f64 {
    let rho = performance_coefficient(lambda, c as f64, mu);
    let p0 = probability_of_zero(lambda, c, mu);
    rho / (1.0 - rho) * p0
}

pub fn average_number_queue_customers(lambda: f64, c: usize, mu: f64) -> f64 {
     average_customer_count(lambda, c, mu) - performance_coefficient(lambda, c as f64, mu)
}

pub fn waiting(lambda: f64, c: usize, mu: f64) -> f64 {
    average_customer_count(lambda, c, mu) / lambda
}

pub fn waiting_queue(lambda: f64, c: usize, mu: f64) -> f64 {
    average_number_queue_customers(lambda, c, mu) / lambda
}
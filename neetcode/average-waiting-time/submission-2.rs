impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut n_customers = customers.len() as u32;
        let mut wait_time = 0u32;
        let mut time = 0u32;

        for c in &customers {
            let arrive = c[0] as u32;
            let cook = c[1] as u32;

            if time < arrive {
                time = arrive + cook;
            } else {
                time += cook;
            }
            wait_time += time - arrive;
        }

        wait_time as f64 / n_customers as f64
    }
}
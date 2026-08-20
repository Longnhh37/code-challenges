impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut n_customers = customers.len() as u32;
        let mut wait_time = 0u32;
        let mut time = 0u32;

        for c in &customers {
            let arrive = c[0] as u32;
            let wait = c[1] as u32;

            if time < arrive {
                time = arrive + wait;
                wait_time += wait;
            } else {
                time += wait;
                wait_time += time - arrive;
            }
        }

        let gcd = Self::gcd(wait_time, n_customers);
        wait_time /= gcd;
        n_customers /= gcd;
        wait_time as f64 / n_customers as f64
    }

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}
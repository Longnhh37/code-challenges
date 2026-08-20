#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![0; (max_weight + 1) as usize];

    for item in items {
        let w = item.weight as usize;
        let v = item.value;

        for j in (w..=max_weight as usize).rev() {
            dp[j] = dp[j].max(v + dp[j - w]);
        }
    }

    dp[max_weight as usize]
}

fn main() {}

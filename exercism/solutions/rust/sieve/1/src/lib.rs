pub fn primes_up_to(lim: u64) -> Vec<u64> {
    let nums = (0..=lim).collect::<Vec<u64>>();
    let lim = lim as usize;
    let mut mark = vec![false; lim + 1];
    let mut res = vec![];

    for i in 2..=lim {
        if !mark[i] {
            res.push(nums[i]);

            let mut j = 2;
            while i * j <= lim {
                mark[i * j] = true;
                j += 1;
            }
        }
    }

    res
}

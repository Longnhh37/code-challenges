fn sort_by_bit(arr: &mut Vec<u32>) {
    arr.sort_unstable_by(|a, b| {
        a.count_ones().cmp(&b.count_ones())
        .then_with(|| a.cmp(&b))
    })
}
​
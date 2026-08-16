fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = arr1.to_vec();
    res.extend(arr2);
    res.sort_unstable();
    res.dedup();
    res
}
​
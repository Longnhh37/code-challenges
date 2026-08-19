fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut res = vec![0; arr.len()];
    let mut write = 0;
    for read in 0..arr.len() {
        if arr[read] > 0 {
            res[write] = arr[read];
            write += 1;
        }
    }
    res
}
​
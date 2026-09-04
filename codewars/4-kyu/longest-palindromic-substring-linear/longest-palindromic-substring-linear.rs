fn longest_palindrome(s: &str) -> String {
    let bytes = s.as_bytes();
    let (mut best_start, mut longest) = (0, 0);
    
    for i in 0..bytes.len() {
        let (cur_start, cur_len) = expand_palin_all(&bytes, i);
        if cur_len > longest {
            (best_start, longest) = (cur_start, cur_len);
        }
    }
    
    String::from_utf8(bytes[best_start..best_start + longest].to_vec()).unwrap()
}
​
fn expand_palin_all(bytes: &[u8], start: usize) -> (usize, usize) {
    let (i1, len1) = expand_palin(bytes, start, start);
    let (i2, len2) = expand_palin(bytes, start, start + 1);
    if len1 >= len2 {
        (i1, len1)
    } else {
        (i2, len2)
    }
}
​
fn expand_palin(bytes: &[u8], left: usize, right: usize) -> (usize, usize) {
    let (mut l, mut r) = (left as isize, right);
    
    while l >= 0 && r < bytes.len() {
        if bytes[l as usize] == bytes[r] {
            l -= 1;
            r += 1;
        } else {
            break;
        }
    }
    
    let l = (l + 1) as usize;
    (l, r - l)
}
​
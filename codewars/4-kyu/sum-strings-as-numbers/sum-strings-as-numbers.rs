pub fn sum_strings(x: &str, y: &str) -> String {
    let is_zero = |s: &str| s.is_empty() || s == "0";
    if is_zero(x) && is_zero(y) {
        return "0".to_string();
    } else if is_zero(x) {
        return y.to_string();
    } else if is_zero(y) {
        return x.to_string();
    }
    
    let mut res: Vec<u8> = Vec::with_capacity(x.len().max(y.len()) + 2);
    
    let mut carry = 0u8;
    let mut bx: Vec<u8> = x.bytes().skip_while(|&b| b == b'0').collect();
    bx.reverse();
    let mut by: Vec<u8> = y.bytes().skip_while(|&b| b == b'0').collect();
    by.reverse();
    let (lx, ly) = (bx.len(), by.len());
    let (mut i, mut j) = (0, 0);
    
    while i < lx || j < ly || carry > 0 {
        let a = if i >= lx { 0 } else { bx[i] - b'0' };
        let b = if j >= ly { 0 } else { by[j] - b'0' };
        let  total = a + b + carry;
        carry = total / 10;
        res.push(total % 10 + b'0');
        i += 1;
        j += 1;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}
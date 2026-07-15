pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();

    let mut total: u64 = 0;

    for c in num_str.chars() {
        let c_int = (c as i32 - '0' as i32) as u64;
        total += c_int.pow(len as u32);
    }
    
    num as u64 == total
}
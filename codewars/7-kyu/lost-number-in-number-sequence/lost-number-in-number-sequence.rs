fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {
    if list.is_empty() {
        return None;
    }
    if list.len() == mixed_list.len() {
        return None;
    }
    
    let mut res = 0;
    for &a in list {
        res ^= a;
    }
    for &a in mixed_list {
        res ^= a;
    }
    Some(res)
}
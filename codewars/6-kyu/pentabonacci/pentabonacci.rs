fn count_odd_pentafib(n: u16) -> u16 {
    let base_parity: [u16; 5] = [0, 1, 1, 0, 0];
    
    let mut odd_cnt = 0u16;
    let mut buf = [0u16; 5];
    
    let limit = n.min(4);
    for i in 0..=limit {
        let p = base_parity[i as usize];
        buf[(i % 5) as usize] = p;
        odd_cnt += p;
    }
    
    if n > 4 {
        for i in 5..=n {
            let idx = (i % 5) as usize;
            let sum_parity: u16 = buf.iter().sum::<u16>() & 1;
            odd_cnt += sum_parity;
            buf[idx] = sum_parity;
        }
    }
    
    if n >= 2 {
        odd_cnt - 1
    } else {
        odd_cnt
    }
}
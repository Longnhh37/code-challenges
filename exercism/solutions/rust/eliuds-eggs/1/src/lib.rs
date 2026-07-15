pub fn egg_count(mut display_value: u32) -> usize {
    let mut cnt = 0;

    while display_value >= 1 {
        let mut pow = 0;

        while 2_u32.pow(pow) <= display_value {
            pow += 1;
        }
        display_value -= 2_u32.pow(pow - 1);

        cnt += 1;
    }

    cnt
}
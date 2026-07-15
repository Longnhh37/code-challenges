pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = vec![];

    for i in 0..5 {
        let bit = (n >> i) & 1;
        match (i, bit) {
            (0, 1) => actions.push("wink"),
            (1, 1) => actions.push("double blink"),
            (2, 1) => actions.push("close your eyes"),
            (3, 1) => actions.push("jump"),
            (4, 1) => actions.reverse(),
            _ => continue,
        }
    }

    actions
}

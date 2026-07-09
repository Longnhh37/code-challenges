pub fn actions(n: u8) -> Vec<&'static str> {
    let cmds = ["wink", "double blink", "close your eyes", "jump"];

    let mut actions: Vec<_> = cmds
        .iter()
        .enumerate()
        .filter_map(|(i, &cmd)| if (n >> i) & 1 == 1 { Some(cmd) } else { None })
        .collect();

    if (n >> 4) & 1 == 1 {
        actions.reverse();
    }

    actions
}

fn main() {
    dbg!(actions(26));
}

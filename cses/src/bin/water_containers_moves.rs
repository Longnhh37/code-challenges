use std::collections::{HashMap, VecDeque, hash_map::Entry};
use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Action {
    FillA,
    FillB,
    EmptyA,
    EmptyB,
    AToB,
    BToA,
    None,
}

impl Action {
    const fn as_str(self) -> &'static str {
        use Action::*;
        match self {
            FillA => "FILL A",
            FillB => "FILL B",
            EmptyA => "EMPTY A",
            EmptyB => "EMPTY B",
            AToB => "MOVE A B",
            BToA => "MOVE B A",
            None => "",
        }
    }
}

type State = (u16, u16);
type Trace = HashMap<State, (u16, u16, u16, Action)>;

fn solve(cap1: u16, cap2: u16, target: u16) -> Option<(State, Trace)> {
    if target > cap1.max(cap2) {
        return None;
    }

    let mut q = VecDeque::new();
    let mut trace = Trace::new();

    q.push_back((0, 0));
    trace.insert((0, 0), (u16::MAX, u16::MAX, 0, Action::None));

    while let Some((a, b)) = q.pop_front() {
        if a == target {
            return Some(((a, b), trace));
        }

        for (da, db, water, act) in neighbors(a, b, cap1, cap2) {
            if let Entry::Vacant(slot) = trace.entry((da, db)) {
                q.push_back((da, db));
                slot.insert((a, b, water, act));
            }
        }
    }

    None
}

fn neighbors(a: u16, b: u16, cap1: u16, cap2: u16) -> Vec<(u16, u16, u16, Action)> {
    use Action::*;

    let mut res = Vec::with_capacity(6);

    // pour a -> b
    let t = a.min(cap2 - b);
    if t != 0 {
        res.push((a - t, b + t, t, AToB));
    }

    // pour b -> a
    let t = b.min(cap1 - a);
    if t != 0 {
        res.push((a + t, b - t, t, BToA));
    }

    // fill
    res.push((cap1, b, cap1 - a, FillA));
    res.push((a, cap2, cap2 - b, FillB));

    // empty
    res.push((0, b, a, EmptyA));
    res.push((a, 0, b, EmptyB));

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u16>().unwrap());

    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let target = it.next().unwrap();

    match solve(a, b, target) {
        None => println!("-1"),
        Some((end, trace)) => {
            let mut moves = Vec::new();
            let mut total: u32 = 0;
            let mut cur = end;

            while let Some(&(pa, pb, water, act)) = trace.get(&cur) {
                if matches!(act, Action::None) {
                    break;
                }
                moves.push(act);
                total += u32::from(water);
                cur = (pa, pb);
            }
            moves.reverse();

            let mut out = String::new();
            out.push_str(&format!("{} {}\n", moves.len(), total));
            for act in moves {
                out.push_str(act.as_str());
                out.push('\n');
            }
            println!("{out}");
        }
    }
}

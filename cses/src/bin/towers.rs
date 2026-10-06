use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = it.next().unwrap() as usize;

    let mut towers = Vec::new();

    for block in it.take(n) {
        let i = towers.partition_point(|&top| top <= block);
        match towers.get_mut(i) {
            Some(top) => *top = block,
            None => towers.push(block),
        }
    }

    println!("{}", towers.len());
}

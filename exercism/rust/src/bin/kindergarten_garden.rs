pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let idx = STUDENTS.iter().position(|&name| name == student).unwrap();

    let mut lines = diagram.lines();
    let front = lines.next().unwrap().as_bytes();
    let back = lines.next().unwrap().as_bytes();

    fn plant(c: u8) -> &'static str {
        match c {
            b'G' => "grass",
            b'C' => "clover",
            b'R' => "radishes",
            b'V' => "violets",
            _ => unreachable!(),
        }
    }

    let i = 2 * idx;

    vec![
        plant(front[i]),
        plant(front[i + 1]),
        plant(back[i]),
        plant(back[i + 1]),
    ]
}

fn main() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let out = plants(diagram, student);
    println!("{out:?}");
}

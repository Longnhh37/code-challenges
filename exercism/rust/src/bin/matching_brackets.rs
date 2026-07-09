pub fn brackets_are_balanced(s: &str) -> bool {
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                match (c, stack.pop()) {
                    ('}', Some('{')) |
                    (']', Some('[')) |
                    (')', Some('(')) => {}
                    _ => return false,
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    let s = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)";
    let out = brackets_are_balanced(s);
    println!("{out}");
}

pub fn build_proverb(list: &[&str]) -> String {
    let pre: &str = "For want of a ";
    let post: &str = " was lost.\n";
    let end: &str = "And all for the want of a ";
    
    let len = list.len() - 1;
    let mut out: String = String::new();

    for i in 0..len {
        out.push_str(pre);
        out.push_str(list[i]);
        out.push_str(" the ");
        out.push_str(list[i + 1]);
        out.push_str(post);
    }
    out.push_str(end);
    out.push_str(list[0]);
    out.push('.');

    out
}

fn main() {
    let proverb_list = vec!["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"];
    let out = build_proverb(&proverb_list);
    println!("{out}");
}

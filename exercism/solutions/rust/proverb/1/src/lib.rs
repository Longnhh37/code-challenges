pub fn build_proverb(list: &[&str]) -> String {
    let pre: &str = "For want of a ";
    let post: &str = " was lost.\n";
    let end: &str = "And all for the want of a ";
    
    let len = list.len();
    let mut out: String = String::new();

    if len == 0 {
        return out;
    }

    for i in 0..(len - 1) {
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

fn multi_greet(names: &[&str]) -> String {
    let mut res = "Hello".to_string();
    let n = names.len();

    if n == 0 {
        res.push_str(", world")
    } else if n == 1 {
        res.push_str(", ");
        res.push_str(names[0]);
    } else {
        for i in 0..n - 1 {
            res.push_str(", ");
            res.push_str(names[i]);
        }
        res.push_str(&format!(" and {}", names[n - 1]));
    }

    res.push('!');
    res
}


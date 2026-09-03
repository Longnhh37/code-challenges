fn no_space(mut x : String) -> String{
    x.retain(|c| !c.is_whitespace());
    x
}
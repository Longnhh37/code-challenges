fn merge<'a>(xs: &'a Vec<usize>, ys: &'a Vec<usize>) -> Vec<&'a usize> {
    xs.iter().chain(ys).collect()
}
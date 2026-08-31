pub fn release_ions<F>(count: usize, mut dispenser: F)
where
    F: FnMut(),
{
    for _ion_no in 1..=count {
        dispenser();
    }
}
​
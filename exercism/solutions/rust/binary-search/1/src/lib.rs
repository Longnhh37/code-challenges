pub fn find<T, C>(array: C, target: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();

    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match array[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Less => low = mid + 1,
        }
    }

    None
}

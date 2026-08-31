fn count_duplicates(text: &str) -> u32 {
    let mut cnt = [0; 128];
    for b in text.bytes() {
        let b = b.to_ascii_lowercase();
        cnt[b as usize] += 1;
    }
    cnt.iter().filter(|&&c| c > 1).count() as u32
}
​
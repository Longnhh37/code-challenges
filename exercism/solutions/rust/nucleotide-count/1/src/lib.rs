use std::collections::HashMap;

#[inline]
fn idx(b: u8) -> Option<usize> {
    match b {
        b'A' => Some(0),
        b'C' => Some(1),
        b'G' => Some(2),
        b'T' => Some(3),
        _ => None,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let target = nucleotide as u8;

    let target_idx = idx(target).ok_or(nucleotide)?;

    let mut arr = [0usize; 4];

    for b in dna.bytes() {
        match idx(b) {
            Some(i) => arr[i] += 1,
            None => return Err(b as char),
        }
    }

    Ok(arr[target_idx])
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut arr = [0usize; 4];

    for b in dna.bytes() {
        match idx(b) {
            Some(i) => arr[i] += 1,
            None => return Err(b as char),
        }
    }

    Ok(HashMap::from([
        ('A', arr[0]),
        ('C', arr[1]),
        ('G', arr[2]),
        ('T', arr[3]),
    ]))
}

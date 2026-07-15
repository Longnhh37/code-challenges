pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let bytes = rna.as_bytes();
    let len = bytes.len();
    
    let mut out: Vec<&str> = Vec::with_capacity(len / 3);
    
    let mut i = 0;
    while i + 2 < len {
       match (bytes[i], bytes[i + 1], bytes[i + 2]) {
            (b'A', b'U', b'G') => out.push("Methionine"),
            (b'U', b'U', b'U') | (b'U', b'U', b'C') => out.push("Phenylalanine"),
            (b'U', b'U', b'A') | (b'U', b'U', b'G') => out.push("Leucine"),
           
            (b'U', b'C', b'U') | (b'U', b'C', b'C') |
            (b'U', b'C', b'A') | (b'U', b'C', b'G') => out.push("Serine"),
           
            (b'U', b'A', b'U') | (b'U', b'A', b'C') => out.push("Tyrosine"),
            (b'U', b'G', b'U') | (b'U', b'G', b'C') => out.push("Cysteine"),
            (b'U', b'G', b'G') => out.push("Tryptophan"),
           
            (b'U', b'A', b'A') | (b'U', b'A', b'G') | (b'U', b'G', b'A') => {
                return Some(out);
            }

            _ => return None,
        }

        i += 3;
    }

    if i != len {
        None
    } else {
        Some(out)
    }
}

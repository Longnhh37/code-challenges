pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut out: Vec<&str> = Vec::with_capacity(rna.len() * 2);
    let bytes = rna.as_bytes();

    for i in 0..bytes.len() - 3 {
        let codon = &bytes[i..i + 3];

        match codon {
            b"AUG" => out.push("Methionine"),
            b"UUU" | b"UUC" => out.push("Phenylalanine"),
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => out.push("Serine"),
            b"UAU" | b"UAC" => out.push("Tyrosine"),
            b"UGU" | b"UGC" => out.push("Cysteine"),
            b"UGG" => out.push("Tryptophan"),
            b"UAA" | b"UAG" | b"UGA" => return Some(out),
            _ => return None,
        }
    }

    Some(out)
}
fn main() {}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: Vec<u8>,
}

impl Dna {
    pub fn new(sequence: &str) -> Result<Dna, usize> {
        let mut dna: Vec<u8> = Vec::with_capacity(sequence.len());

        for (i, b) in sequence.bytes().enumerate() {
            match b {
                b'A' | b'C' | b'T' | b'G' => dna.push(b),
                _ => return Err(i),
            }
        }

        Ok(Self { dna })
    }

    pub fn into_rna(self) -> Rna {
        let mut rna: Vec<u8> = Vec::with_capacity(self.dna.len());

        for b in self.dna.iter() {
            match b {
                b'G' => rna.push(b'C'),
                b'C' => rna.push(b'G'),
                b'T' => rna.push(b'A'),
                b'A' => rna.push(b'U'),
                _ => unreachable!(),
            }
        }

        Rna { rna }
    }
}

impl Rna {
    pub fn new(sequence: &str) -> Result<Rna, usize> {
        let mut rna: Vec<u8> = Vec::with_capacity(sequence.len());

        for (i, b) in sequence.bytes().enumerate() {
            match b {
                b'A' | b'C' | b'U' | b'G' => rna.push(b),
                _ => return Err(i),
            }
        }

        Ok(Self { rna })
    }
}

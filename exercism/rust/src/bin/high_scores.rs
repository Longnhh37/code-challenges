#[derive(Debug)]
pub struct HighScores {
    records: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self { records: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        &self.records
    }

    pub fn latest(&self) -> Option<u32> {
        self.records.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.records.iter().copied().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut tmp = self.records.clone();

        if tmp.len() <= 3 {
            tmp.sort_unstable_by(|a, b| b.cmp(a));
            return tmp
        }

        let (top3, _, _) = tmp.select_nth_unstable_by(3, |a, b| b.cmp(a));
        top3.sort_unstable_by(|a, b| b.cmp(a));
        top3.to_vec()
    }
}
fn main() {}

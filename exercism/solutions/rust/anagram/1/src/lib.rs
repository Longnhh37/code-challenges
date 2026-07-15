use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut out: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());
    
    for &candidate in possible_anagrams {
        if candidate.len() != word.len() || candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        if normalize(candidate) == normalize(word) && word != candidate {
            out.insert(candidate);
        }
    }

    out
}

fn normalize(s: &str) -> Vec<char> {
    let mut v: Vec<char> = s.to_lowercase().chars().collect();
    v.sort_unstable();
    v
}

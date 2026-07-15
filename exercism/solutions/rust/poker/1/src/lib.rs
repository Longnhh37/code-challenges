use std::collections::HashSet;

pub fn winning_hands<'a>(hands: &[&'a str]) -> HashSet<&'a str> {
    let mut best = [0u8; 6];
    let mut result = HashSet::new();

    for &hand in hands {
        let (ranks, same_suit) = parse_hand(hand);
        let score = rank_hand(ranks, same_suit);

        if score > best {
            best = score;
            result.clear();
            result.insert(hand);
        } else if score == best {
            result.insert(hand);
        }
    }

    result
}

fn parse_hand(hand: &str) -> (Vec<u8>, bool) {
    let mut ranks = Vec::with_capacity(5);
    let mut suits = Vec::with_capacity(5);

    for card in hand.split_ascii_whitespace() {
        let mut chars = card.chars();

        let rank = match chars.next().unwrap() {
            '1' => {
                chars.next();
                10
            }
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => c as u8 - b'0',
        };

        let suit = chars.next().unwrap();
        ranks.push(rank);
        suits.push(suit);
    }

    let same_suit = suits.iter().all(|&s| s == suits[0]);
    (ranks, same_suit)
}

fn normalize(mut ranks: Vec<u8>) -> Vec<u8> {
    ranks.sort_unstable_by(|a, b| b.cmp(a));

    if ranks == [14, 5, 4, 3, 2] {
        return vec![5, 4, 3, 2, 1];
    }

    ranks
}

fn build_count(ranks: &[u8]) -> [u8; 15] {
    let mut count = [0; 15];
    for &r in ranks {
        count[r as usize] += 1;
    }

    count
}

fn is_straight(ranks: &[u8]) -> bool {
    ranks.windows(2).all(|w| w[0] == w[1] + 1)
}

fn rank_hand(ranks: Vec<u8>, same_suit: bool) -> [u8; 6] {
    let ranks = normalize(ranks);
    let count = build_count(&ranks);

    let mut groups: Vec<(u8, u8)> = count
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c > 0)
        .map(|(r, &c)| (c, r as u8))
        .collect();

    groups.sort_unstable_by(|a, b| b.cmp(a));

    let is_seq = is_straight(&ranks);

    let mut out = [0; 6];

    match (is_seq, same_suit, groups.as_slice()) {
        // royal, straight flush
        (true, true, _) => {
            out[0] = if ranks[0] == 14 { 10 } else { 9 };
            out[1] = ranks[0];
        }

        // four of a kind
        (_, _, [(4, r), (1, k)]) => {
            out[0] = 8;
            out[1] = *r;
            out[2] = *k;
        }

        // full house
        (_, _, [(3, r), (2, p)]) => {
            out[0] = 7;
            out[1] = *r;
            out[2] = *p;
        }

        // flush
        (false, true, _) => {
            out[0] = 6;
            out[1..6].copy_from_slice(&ranks);
        }

        // straight
        (true, false, _) => {
            out[0] = 5;
            out[1] = ranks[0];
        }

        // three of a kind
        (_, _, [(3, r), (1, k1), (1, k2)]) => {
            out[0] = 4;
            out[1] = *r;
            out[2] = (*k1).max(*k2);
            out[3] = (*k1).min(*k2);
        }

        // two pair
        (_, _, [(2, r1), (2, r2), (1, k)]) => {
            out[0] = 3;
            out[1] = (*r1).max(*r2);
            out[2] = (*r1).min(*r2);
            out[3] = *k;
        }

        // pair
        (_, _, [(2, r), ..]) => {
            out[0] = 2;
            out[1] = *r;

            let mut i = 2;
            for &(cnt, rank) in &groups {
                if cnt == 1 {
                    out[i] = rank;
                    i += 1;
                }
            }
        }

        // high card
        _ => {
            out[0] = 1;
            out[1..6].copy_from_slice(&ranks);
        }
    }

    out
}

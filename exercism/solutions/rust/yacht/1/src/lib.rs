#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(mut dice: Dice, category: Category) -> u8 {
    let mut counter = [0u8; 7];
    for v in dice {
        counter[v as usize] += 1;
    }

    match category {
        Category::Ones => dice.iter().filter(|&&v| v == 1).count() as u8,
        Category::Twos => 2 * dice.iter().filter(|&&v| v == 2).count() as u8,
        Category::Threes => 3 * dice.iter().filter(|&&v| v == 3).count() as u8,
        Category::Fours => 4 * dice.iter().filter(|&&v| v == 4).count() as u8,
        Category::Fives => 5 * dice.iter().filter(|&&v| v == 5).count() as u8,
        Category::Sixes => 6 * dice.iter().filter(|&&v| v == 6).count() as u8,
        Category::FullHouse => {
            if counter.contains(&3) && counter.contains(&2) {
                dice.iter().sum::<u8>()
            } else {
                0
            }
        }
        Category::FourOfAKind => counter
            .iter()
            .enumerate()
            .find_map(|(i, &v)| if v >= 4 { Some(i as u8 * 4) } else { None })
            .unwrap_or(0),
        Category::LittleStraight => {
            dice.sort();
            if dice.iter().enumerate().all(|(i, &v)| v == i as u8 + 1) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            dice.sort();
            if dice.iter().enumerate().all(|(i, &v)| v == i as u8 + 2) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum::<u8>(),
        Category::Yacht => {
            if counter.contains(&5) {
                50
            } else {
                0
            }
        }
    }
}

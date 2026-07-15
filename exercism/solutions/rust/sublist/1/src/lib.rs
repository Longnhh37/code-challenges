#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;
    let len1 = first_list.len();
    let len2 = second_list.len();
    
    if first_list == second_list {
        return Equal;
    }

    if len1 == 0 {
        return Sublist;
    }

    if len2 == 0 {
        return Superlist;
    }

    match len1.cmp(&len2) {
        std::cmp::Ordering::Less => {
            if second_list.windows(len1).any(|w| w == first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        std::cmp::Ordering::Greater => {
            if first_list.windows(len2).any(|w| w == second_list) {
                Superlist
            } else {
                Unequal
            }
        }
        std::cmp::Ordering::Equal => Unequal,
    }
}
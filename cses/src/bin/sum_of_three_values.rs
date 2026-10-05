use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let target = it.next().unwrap();
    let mut nums: Vec<_> = it.take(n).collect();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        map.entry(n).or_default().push(i + 1);
    }

    nums.sort_unstable();

    let mut res: Option<Vec<usize>> = None;
    'outer: for (i, &num) in nums.iter().enumerate() {
        if num >= target {
            break;
        }
        let (mut l, mut r) = (i + 1, n - 1);
        while l < r {
            let sum = num + nums[l] + nums[r];
            if sum == target {
                res = Some(vec![num, nums[l], nums[r]]);
                break 'outer;
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    match res {
        None => println!("IMPOSSIBLE"),
        Some(v) => {
            let [a, b, c] = v[..3] else { unreachable!() };
            let a = map.get_mut(&a).unwrap().pop().unwrap();
            let b = map.get_mut(&b).unwrap().pop().unwrap();
            let c = map.get_mut(&c).unwrap().pop().unwrap();
            println!("{} {} {}", a, b, c);
        }
    }
}

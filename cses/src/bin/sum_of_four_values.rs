use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    if n < 4 {
        println!("IMPOSSIBLE");
        return;
    }
    let target = it.next().unwrap();
    let mut nums: Vec<_> = it.take(n).collect();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        map.entry(x).or_default().push(i + 1);
    }

    nums.sort_unstable();

    let mut res: Option<[usize; 4]> = None;
    'outer: for i in 0..n - 3 {
        if nums[i] + nums[i + 1] >= target {
            break;
        }
        for j in i + 1..n - 2 {
            if nums[i] + nums[j] >= target {
                break;
            }
            let (mut l, mut r) = (j + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[j] + nums[l] + nums[r];
                if sum == target {
                    res = Some([nums[i], nums[j], nums[l], nums[r]]);
                    break 'outer;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
    }

    match res {
        None => println!("IMPOSSIBLE"),
        Some(v) => {
            let [a, b, c, d] = v;
            let a = map.get_mut(&a).unwrap().pop().unwrap();
            let b = map.get_mut(&b).unwrap().pop().unwrap();
            let c = map.get_mut(&c).unwrap().pop().unwrap();
            let d = map.get_mut(&d).unwrap().pop().unwrap();
            println!("{} {} {} {}", a, b, c, d);
        }
    }
}

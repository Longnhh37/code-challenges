#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 10],
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, digits: &[u8]) {
        let mut node = self;
        for &d in digits {
            node = node.children[d as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
    }

    fn longest_common_prefix_len(&self, digits: &[u8]) -> usize {
        let mut node = self;
        let mut len = 0;
        for &d in digits {
            match node.children[d as usize].as_deref() {
                Some(next) => { node = next; len += 1; }
                None => break,
            }
        }
        len
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        fn to_digits(n: i32) -> Vec<u8> {
            n.to_string().bytes().map(|b| b - b'0').collect()
        }

        let digits1: Vec<Vec<u8>> = arr1
            .into_iter()
            .map(|n| to_digits(n))
            .collect();

        let digits2: Vec<Vec<u8>> = arr2
            .into_iter()
            .map(|n| to_digits(n))
            .collect();
        
        let (build_list, query_list) = if digits1.len() <= digits2.len() {
            (&digits1, &digits2)
        } else {
            (&digits2, &digits1)
        };

        let mut root = TrieNode::new();
        for d in build_list {
            root.insert(d);
        }

        let mut res = 0;
        for d in query_list {
            let len = root.longest_common_prefix_len(d);
            res = res.max(len);
        }

        res as i32
    }
}

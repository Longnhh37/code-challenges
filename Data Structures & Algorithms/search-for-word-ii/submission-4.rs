const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Default)]
struct Node {
    word: Option<String>,
    children: [Option<Box<Node>>; 26],
}

#[derive(Default)]
struct PrefixTree {
    root: Box<Node>,
}
impl PrefixTree {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, word: &str) {
        let mut cur = &mut self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(Node::default()));
        }
        cur.word = Some(word.to_string());
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut tree = PrefixTree::new();
        for word in &words {
            tree.insert(word);
        }
        let mut res = Vec::new();
        let rows = board.len();
        let cols = if rows > 0 { board[0].len() } else { 0 };

        for r in 0..rows {
            for c in 0..cols {
                let b = board[r][c] as u8;
                if b == b'#' || !b.is_ascii_lowercase() {
                    continue;
                }
                let idx = (b - b'a') as usize;
                if tree.root.children[idx].is_some() {
                    Self::dfs(&mut board, r as i32, c as i32, &mut tree.root, &mut res);
                }
            }
        }
        res
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: i32, c: i32, node: &mut Node, res: &mut Vec<String>) {
        let rows = board.len() as i32;
        let cols = board[0].len() as i32;
        if r < 0 || c < 0 || r >= rows || c >= cols {
            return;
        }
        let ch = board[r as usize][c as usize];
        if ch == '#' {
            return;
        }
        let idx = (ch as u8 - b'a') as usize;
        let next = match &mut node.children[idx] {
            Some(n) => n,
            None => return,
        };

        if let Some(word) = next.word.take() {
            res.push(word);
        }

        board[r as usize][c as usize] = '#';
        for (dr, dc) in DIRS {
            Self::dfs(board, r + dr, c + dc, next, res);
        }
        board[r as usize][c as usize] = ch;
    }
}
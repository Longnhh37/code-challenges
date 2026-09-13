struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            word: None,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut node = self;
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            node = node.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.word = Some(s.to_string());
    }
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for w in &words {
            trie.insert(w);
        }       

        let (rows, cols) = (board.len(), board[0].len());
        let mut res = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                Self::backtrack(&mut board, r, c, &mut trie, &mut res);
            }
        }

        res
    }
    
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        r: usize,
        c: usize,
        trie: &mut Trie,
        res: &mut Vec<String>,
    ) {
        let ch = board[r][c];
        if ch == '#' {
            return;
        } 
        let i = (ch as u8 - b'a') as usize;

        let Some(child) = trie.children[i].as_mut() else { return };
        if let Some(w) = child.word.take() {
            res.push(w);
        }
        
        board[r][c] = '#';
        let (irows, icols) = (board.len() as i32, board[0].len() as i32);

        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                Self::backtrack(board, nr as usize, nc as usize, child, res);
            }
        }
        
        board[r][c] = ch;
        
        if child.children.iter().all(|c| c.is_none()) && child.word.is_none() {
            trie.children[i] = None;
        }
    }
}

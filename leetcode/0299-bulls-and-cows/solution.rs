impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut sec = vec![0; 10];
        let mut gue = vec![0; 10];

        for (b1, b2) in secret.bytes().zip(guess.bytes()) {
            if b1 == b2 {
                bulls += 1;
                continue;
            }
            sec[(b1 - b'0') as usize] += 1;
            gue[(b2 - b'0') as usize] += 1;
        }

        let mut cows = 0;
        for i in 0..10 {
            cows += sec[i].min(gue[i]);
        }

        format!("{}A{}B", bulls, cows)
    }
}

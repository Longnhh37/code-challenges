use unicode_segmentation::UnicodeSegmentation;

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: usize) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let rails = self.0;
        if rails <= 1 || text.is_empty() {
            return text.to_string();
        }

        let graphemes: Vec<&str> = text.graphemes(true).collect();
        let len = graphemes.len();
        let row = rails - 1;

        let mut out = String::new();
        out.reserve(text.len());

        for cur in 0..=row {
            let (jump1, jump2) = if cur == 0 || cur == row {
                (row * 2, row * 2)
            } else {
                ((row - cur) * 2, cur * 2)
            };

            let mut i = cur;

            while i < len {
                out.push_str(graphemes[i]);

                if jump1 == 0 {
                    break;
                }
                i += jump1;
                if i >= len {
                    break;
                }

                out.push_str(graphemes[i]);

                if jump2 == 0 {
                    break;
                }
                i += jump2;
            }
        }

        out
    }

    pub fn decode(&self, cipher: &str) -> String {
        let rails = self.0;
        if rails <= 1 || cipher.is_empty() {
            return cipher.to_string();
        }

        let graphemes: Vec<&str> = cipher.graphemes(true).collect();
        let len = graphemes.len();
        let row = rails - 1;

        let mut out: Vec<&str> = vec![""; len];
        let mut iter = graphemes.into_iter();

        for cur in 0..=row {
            let (jump1, jump2) = if cur == 0 || cur == row {
                (row * 2, row * 2)
            } else {
                ((row - cur) * 2, cur * 2)
            };

            let mut i = cur;

            while i < len {
                out[i] = iter.next().unwrap();

                if jump1 == 0 {
                    break;
                }
                i += jump1;
                if i >= len {
                    break;
                }

                out[i] = iter.next().unwrap();

                if jump2 == 0 {
                    break;
                }
                i += jump2;
            }
        }

        out.concat()
    }
}

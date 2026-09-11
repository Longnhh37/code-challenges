const MAX_TARGET: usize = 10_000;
const WORDS: usize = (MAX_TARGET / 64) + 1;
impl Solution {
    #[inline]
    fn or_shift_left_inplace(bits: &mut [u64; WORDS], shift: usize) {
        let word_shift = shift / 64;
        let bit_shift = shift % 64;

        if bit_shift == 0 {
            for i in (word_shift..WORDS).rev() {
                bits[i] |= bits[i - word_shift];
            }
        } else {
            for i in (word_shift..WORDS).rev() {
                let low = bits[i - word_shift] << bit_shift;
                let high = if i > word_shift {
                    bits[i - word_shift - 1] >> (64 - bit_shift)
                } else {
                    0
                };
                bits[i] |= low | high;
            }
        }
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: u32 = nums.iter().map(|&x| x as u32).sum();
        if total & 1 == 1 {
            return false;
        }
        let target = (total / 2) as usize;

        let mut bits = [0u64; WORDS];
        bits[0] = 1;

        for n in nums {
            Self::or_shift_left_inplace(&mut bits, n as usize);
        }

        let word = target / 64;
        let bit = target % 64;
        (bits[word] >> bit) & 1 == 1
    }
}

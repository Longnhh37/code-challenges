impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
       let mut i = digits.len() - 1; 
       let mut carry = 1;

        loop {
        let total = digits[i] + carry;
        digits[i] = total % 10;
        carry = total / 10;
        if carry == 0 {
            break;
        }

        if i != 0 {
            i -= 1;
        } else {
            digits.insert(0, carry);
            break;
        }
    }

    digits
    }
}

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        num.reverse();
        let mut num2 = Vec::new();
        while k > 0 {
            num2.push(k % 10);
            k /= 10;
        }
        if num.len() < num2.len() {
            std::mem::swap(&mut num, &mut num2);
        }

        let mut carry = 0;
        let mut total = 0;
        let mut res = Vec::new();
        for i in 0..num2.len() {
            total = num[i] + num2[i] + carry;
            carry = total / 10;
            res.push(total % 10);
        }
        for j in num2.len()..num.len() {
            total = num[j] + carry;
            carry = total / 10;
            res.push(total % 10);
        }

        if carry > 0 {
            res.push(carry);
        }

        res.reverse();
        res
    }
}

// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
pub struct Solution {}

impl Solution {
    // O(log n)
    // O(1)
    pub fn v1(num: i32) -> i32 {
        let mut steps = 0;
        let mut v_number = num;
        while v_number != 0 {
            v_number = match v_number % 2 == 0 {
                true => v_number / 2,
                false => v_number - 1,
            };
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        assert_eq!(Solution::v1(14), 6);
        assert_eq!(Solution::v1(8), 4);
        assert_eq!(Solution::v1(123), 12);
    }
}

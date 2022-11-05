// https://leetcode.com/problems/fizz-buzz/
pub struct Solution {}

impl Solution {
    // O(n)
    // O(1)
    pub fn v1(n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(n as usize);
        for i in 1..=n {
            res.push(match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                (_, _) => i.to_string(),
            })
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        assert_eq!(Solution::v1(3,), vec!["1", "2", "Fizz"]);
        assert_eq!(Solution::v1(5,), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            Solution::v1(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}

// https://leetcode.com/problems/richest-customer-wealth/
pub struct Solution {}

impl Solution {
    // O(M*N)
    // O(1)
    pub fn v1(accounts: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for account in accounts {
            res = res.max(account.iter().sum());
        }
        res
    }
    pub fn v2(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        assert_eq!(Solution::v1(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6,);
        assert_eq!(Solution::v1(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10,);
        assert_eq!(
            Solution::v1(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17,
        );
    }
    #[test]
    fn test_v2() {
        assert_eq!(Solution::v2(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6,);
        assert_eq!(Solution::v2(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10,);
        assert_eq!(
            Solution::v2(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17,
        );
    }
}

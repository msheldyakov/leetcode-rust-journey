// https://leetcode.com/problems/running-sum-of-1d-array/
pub struct Solution {}

impl Solution {
    // O(n)
    // O(1)
    pub fn v1(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut temp = 0;
        for i in nums {
            temp += i;
            res.push(temp);
        }
        res
    }
    pub fn v2(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, i| {
                *acc += i;
                Some(*acc)
            })
            .collect()
    }
    pub fn v3(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .scan(0, |acc, i| {
                *acc += i;
                Some(*acc)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use easybench::bench;

    #[test]
    fn test_v1() {
        assert_eq!(Solution::v1(vec![1, 2, 3, 4]), vec![1, 3, 6, 10],);
        assert_eq!(Solution::v1(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5],);
        assert_eq!(Solution::v1(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17],);
    }
    #[test]
    fn test_v2() {
        assert_eq!(Solution::v2(vec![1, 2, 3, 4]), vec![1, 3, 6, 10],);
        assert_eq!(Solution::v2(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5],);
        assert_eq!(Solution::v2(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17],);
    }
    #[test]
    fn test_v3() {
        assert_eq!(Solution::v3(vec![1, 2, 3, 4]), vec![1, 3, 6, 10],);
        assert_eq!(Solution::v3(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5],);
        assert_eq!(Solution::v3(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17],);
    }
    #[test]
    // into_iter v3 faster than iter?
    fn test_bench() {
        println!("v2: {}", bench(|| Solution::v2((0..10000).collect())));
        println!("v3: {}", bench(|| Solution::v3((0..10000).collect())));
    }
}

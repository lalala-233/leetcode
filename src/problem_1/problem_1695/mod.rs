pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    // too slow
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut current = 0;
        let mut map = HashMap::new();
        nums.into_iter()
            .enumerate()
            .map(|(index, value)| {
                if let Some(old) = map.insert(value, index) {
                    current = old.max(current);
                }
                map.iter()
                    .filter(|(_, index)| **index >= current)
                    .map(|(value, _)| value)
                    .sum()
            })
            .max()
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(nums: &[i32], expect: i32) {
        let solution = Solution::maximum_unique_subarray(nums.to_vec());
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(&[4, 2, 4, 5, 6], 17);
    }
    #[test]
    fn test_2() {
        test(&[5, 2, 1, 2, 5, 2, 1, 2, 5], 8);
    }
    #[test]
    fn test_3() {
        test(&[1], 1);
    }
}

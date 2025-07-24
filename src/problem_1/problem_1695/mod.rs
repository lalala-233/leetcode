pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::cast_possible_truncation)]
    // slow and high memory
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut current = 0;
        let mut current_index = 0;
        let mut iter = nums.iter();
        nums.iter()
            .enumerate()
            .map(|(index, value)| {
                current += value;
                if let Some(old) = map.insert(value, index) {
                    if old >= current_index {
                        current_index = old + 1;
                        for i in iter.by_ref() {
                            current -= i;
                            if i == value {
                                break;
                            }
                        }
                    }
                }
                current
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
    #[test]
    fn test_4() {
        test(&[1, 2, 4, 1, 1, 3], 7);
    }
    #[test]
    fn test_5() {
        test(&[1, 2, 4, 1, 1, 3, 4], 8);
    }
    #[test]
    fn test_6() {
        test(&[1, 2, 4, 1, 1, 9], 10);
    }
}

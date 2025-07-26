pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut left_height = 0;
        let mut right = heights.len() as i32;
        let mut right_height = 0;
        let mut result = 0;
        let mut iter = heights.into_iter().enumerate();
        loop {
            if left_height > right_height {
                let Some((index, height)) =
                    iter.by_ref().rev().find(|(_, value)| *value > right_height)
                else {
                    break;
                };
                right = index as i32;
                right_height = height;
            } else {
                let Some((index, height)) = iter.by_ref().find(|(_, value)| *value > left_height)
                else {
                    break;
                };
                left = index as i32;
                left_height = height;
            }
            result = result.max((right - left) * right_height.min(left_height));
        }
        result
    }
}
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::needless_pass_by_value)]
// too slow
// though you can skip some needless calculation by filter, but it is still slow.
pub fn max_area(heights: Vec<i32>) -> i32 {
    heights
        .iter()
        .enumerate()
        .filter_map(|(left, height_left)| {
            heights
                .iter()
                .enumerate()
                .skip(left + 1)
                .map(move |(right, height_right)| {
                    (right - left) as i32 * height_left.min(height_right)
                })
                .max()
        })
        .max()
        .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(heights: &[i32], expect: i32) {
        let solution = Solution::max_area(heights.to_vec());
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(&[1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    }
    #[test]
    fn test_2() {
        test(&[1, 1], 1);
    }
    #[test]
    fn test_3() {
        test(&[1, 2, 4, 3], 4);
    }
}

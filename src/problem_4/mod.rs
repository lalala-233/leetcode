pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::stable_sort_primitive)]
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        // nums1.extend(nums2); more slow, why?
        nums1.append(&mut nums2);
        nums1.sort();
        let middle = nums1.len() / 2;
        if nums1.len() % 2 == 1 {
            f64::from(nums1[middle])
        } else {
            f64::from(nums1[middle - 1] + nums1[middle]) / 2.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(nums1: &[i32], nums2: &[i32], expect: f64) {
        let solution = Solution::find_median_sorted_arrays(nums1.to_vec(), nums2.to_vec());
        assert!(solution - expect < 0.1);
    }
    #[test]
    fn test_1() {
        test(&[1, 3], &[2], 2.00000);
    }

    #[test]
    fn test_2() {
        test(&[1, 2], &[2, 4], 2.50000);
    }
}

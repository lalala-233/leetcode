pub mod problem_10;
pub mod problem_1233;
pub mod problem_1695;
pub mod problem_1948;
pub mod problem_1957;
pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // faster than solution_1 but still slow than HashMap (a little)
        // However it saves more memory than solution_1
        // use std::collections::BTreeMap;
        // let mut map = BTreeMap::new();
        // Most fast but save less memory than solution_1 and BTreeMap
        use std::collections::HashMap;
        let mut map = HashMap::new();
        nums.into_iter()
            .enumerate()
            .find_map(|(index_1, value)| {
                if let Some(&index_2) = map.get(&(target - value)) {
                    Some(vec![index_1 as i32, index_2])
                } else {
                    map.insert(value, index_1 as i32);
                    None
                }
            })
            .unwrap_or_default()
    }
}
// It's slow (but acceptable), but it saves memory
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::needless_pass_by_value)]
pub fn solution_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .flat_map(|(index_1, value_1)| {
            nums.iter()
                .enumerate()
                .skip(index_1 + 1)
                .map(move |(index_2, value_2)| (value_1 + value_2, [index_1, index_2]))
        })
        .find(|(result, _)| result == &target)
        .unwrap_or_default()
        .1
        .map(|index| index as i32)
        .to_vec()
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(nums: &[i32], target: i32, expect: &[i32]) {
        let mut solution = Solution::two_sum(nums.to_vec(), target);
        solution.sort_unstable();
        let mut expect = expect.to_vec();
        expect.sort_unstable();
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(&[2, 7, 11, 15], 9, &[0, 1]);
    }
    #[test]
    fn test_2() {
        test(&[3, 2, 4], 6, &[1, 2]);
    }
    #[test]
    fn test_3() {
        test(&[3, 3], 6, &[0, 1]);
    }
}

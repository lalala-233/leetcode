pub struct Solution;
use std::collections::HashMap;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = -1;
        let mut map = HashMap::new();
        s.char_indices()
            .map(|(index, c)| (index as i32, c))
            .map(|(index, c)| {
                match map.insert(c, index) {
                    Some(i) if i > start => {
                        start = i;
                    }
                    _ => (),
                }
                index - start
            })
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(s: &str, expect: i32) {
        let solution = Solution::length_of_longest_substring(s.to_string());
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test("abcabcbb", 3);
    }

    #[test]
    fn test_2() {
        test("bbbbb", 1);
    }
    #[test]
    fn test_3() {
        test("pwwkew", 3);
    }
    #[test]
    fn test_4() {
        test(" ", 1);
    }
    #[test]
    fn test_5() {
        test("abba", 2);
    }
}

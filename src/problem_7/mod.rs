pub struct Solution;
impl Solution {
    #[must_use]
    pub fn reverse(x: i32) -> i32 {
        let s: String = x.to_string().chars().rev().collect();
        s.strip_suffix('-').map_or_else(
            || s.parse::<i32>().unwrap_or_default(),
            |s| -s.parse::<i32>().unwrap_or_default(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(x: i32, expect: i32) {
        let solution = Solution::reverse(x);
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(123, 321);
    }
    #[test]
    fn test_2() {
        test(-123, -321);
    }
    #[test]
    fn test_3() {
        test(120, 21);
    }
    #[test]
    fn test_4() {
        test(0, 0);
    }
    #[test]
    fn test_5() {
        test(10_0000_0003, 0); // 30_0000_0001 > i32::MAX
    }
}

pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.trim_start().chars();
        let mut s = match iter.next() {
            Some(c) if c == '+' || c == '-' || c.is_numeric() => c.to_string(),
            _ => return 0,
        };
        iter.take_while(|c| c.is_numeric()).for_each(|c| s.push(c));
        match s.parse() {
            Ok(i) => i,
            Err(err) => match err.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(s: &str, expect: i32) {
        let solution = Solution::my_atoi(s.to_string());
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test("42", 42);
    }
    #[test]
    fn test_2() {
        test(" -042", -42);
    }
    #[test]
    fn test_3() {
        test("1337c0d3", 1337);
    }
    #[test]
    fn test_4() {
        test("0-1", 0);
    }
    #[test]
    fn test_5() {
        test("words and 987", 0);
    }
    #[test]
    fn test_6() {
        test("+-12", 0);
    }
    #[test]
    fn test_7() {
        test("  0000000000012345678", 1234_5678);
    }
}

pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let binding = x.to_string();
        let chars = binding.chars();
        chars.clone().zip(chars.rev()).all(|(c_1, c_2)| c_1 == c_2)
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(x: i32, expect: bool) {
        let solution = Solution::is_palindrome(x);
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(121, true);
    }
    #[test]
    fn test_2() {
        test(-121, false); // -121 != 121-
    }
    #[test]
    fn test_3() {
        test(10, false);
    }
    #[test]
    fn test_4() {
        test(0, true);
    }
}

pub struct Solution;
impl Solution {
    #[must_use]
    pub fn is_match(_s: String, _pattern: String) -> bool {
        // 写不来一点
        todo!()
    }
}
// #[cfg(test)]
// mod tests {
//     use super::Solution;
//     fn test(s: &str, pattern: &str, expect: bool) {
//         let solution = Solution::is_match(s.to_string(), pattern.to_string());
//         assert_eq!(solution, expect);
//     }
//     #[test]
//     fn test_1() {
//         test("aa", "a", false);
//     }
//     #[test]
//     fn test_2() {
//         test("aa", "a*", true);
//     }
//     #[test]
//     fn test_3() {
//         test("aa", "a.", true);
//     }
//     #[test]
//     fn test_4() {
//         test("aa", ".*", true);
//     }
//     #[test]
//     fn test_5() {
//         test("aab", "c*a*b", true);
//     }
//     #[test]
//     fn test_6() {
//         test("mississippi", "mis*is*ip*.", true);
//     }
//     #[test]
//     fn test_7() {
//         test("aaa", "aaaa", false);
//     }
//     #[test]
//     fn test_8() {
//         test("aaa", "aa*a*aa*a*a", true);
//     }
// }

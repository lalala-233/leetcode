pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_palindrome(s: String) -> String {
        let iter_1 = get_splits(&s).map(|(left, right)| {
            let iter = left.chars().rev().zip(right.chars());
            collect(iter, String::new())
        });
        let iter_2 = get_splits(&s).map(|(left, right)| {
            let mut left_chars = left.chars().rev();
            let init = left_chars
                .next()
                .as_ref()
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            collect(left_chars.zip(right.chars()), init)
        });
        max_by_len(iter_1.chain(iter_2))
    }
}
fn get_splits(s: &str) -> impl Iterator<Item = (&str, &str)> {
    // s consists of ascii_alphanumeric
    (0..=s.len()).map(|i| s.split_at(i))
}
fn collect(iter: impl Iterator<Item = (char, char)>, init: String) -> String {
    iter.take_while(|(left, right)| left == right)
        .fold(init, |mut acc, (left, right)| {
            acc.insert(0, left);
            acc.push(right);
            acc
            // format!("{left}{acc}{right}") too slow
        })
}
fn max_by_len(iter: impl Iterator<Item = String>) -> String {
    iter.max_by_key(std::string::String::len)
        .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(s: &str, expect: &str) {
        let solution = Solution::longest_palindrome(s.to_string());
        assert_eq!(solution, expect.to_string());
    }
    #[test]
    fn test_1() {
        test("babad", "aba"); // or "bab"
    }

    #[test]
    fn test_2() {
        test("abbr", "bb");
    }
    #[test]
    fn test_3() {
        test("a", "a");
    }
    #[test]
    fn test_4() {
        test("", "");
    }
}

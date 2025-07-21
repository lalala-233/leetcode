pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn make_fancy_string(s: String) -> String {
        let mut iter = s.chars();
        iter.next().map_or(String::new(), |first| {
            let mut have_two_same_characters = false;
            let mut current = first;
            iter.fold(first.to_string(), |mut acc, c| {
                if c == current {
                    if !have_two_same_characters {
                        have_two_same_characters = true;
                        acc.push(c);
                    }
                } else {
                    have_two_same_characters = false;
                    acc.push(c);
                }
                current = c;
                acc
            })
        })
    }
}
#[must_use]
#[allow(clippy::needless_pass_by_value)]
// Hmmm, it is slower than solution though they have same logic.
pub fn make_fancy_string(s: String) -> String {
    let mut iter = s.chars();
    iter.next().map_or(String::new(), |first| {
        get_fancy_string(iter, first.to_string(), first, false)
    })
}
fn get_fancy_string(
    mut iter: impl Iterator<Item = char>,
    mut result: String,
    current: char,
    have_two_same_characters: bool,
) -> String {
    match iter.next() {
        Some(c) if c == current && have_two_same_characters => {
            get_fancy_string(iter, result, c, true)
        }
        Some(c) => {
            result.push(c);
            if c == current {
                get_fancy_string(iter, result, c, true)
            } else {
                get_fancy_string(iter, result, c, false)
            }
        }
        None => result,
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(s: &str, expect: &str) {
        let solution = Solution::make_fancy_string(s.to_string());
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test("leeetcode", "leetcode");
    }
    #[test]
    fn test_2() {
        test("aaabaaaa", "aabaa");
    }
    #[test]
    fn test_3() {
        test("aab", "aab");
    }
}

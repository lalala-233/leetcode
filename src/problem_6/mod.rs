pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut strings = Strings::new(num_rows);
        s.char_indices().for_each(|c| strings.push(c));
        strings.to_string()
    }
}
// from internet
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::needless_pass_by_value)]
pub fn solution_2(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    let mut rows = vec![String::new(); num_rows];
    let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
    iter.zip(s.chars()).for_each(|(i, c)| rows[i].push(c));
    rows.concat()
}
struct Strings {
    string: Vec<String>,
    num_rows: usize,
}
impl Strings {
    fn new(num_rows: usize) -> Self {
        Self {
            string: vec![String::new(); num_rows],
            num_rows,
        }
    }
    pub fn push(&mut self, (index, c): (usize, char)) {
        let current = self.current(index);
        self.string[current].push(c);
    }
    const fn current(&self, index: usize) -> usize {
        let index_max = self.num_rows - 1;
        if index_max == 0 {
            return 0;
        }
        let is_down = index % (index_max * 2) < index_max;
        if is_down {
            index % index_max
        } else {
            index_max - (index % index_max)
        }
    }
}
impl std::fmt::Display for Strings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.concat())
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(s: &str, num_rows: i32, expect: &str) {
        let solution = Solution::convert(s.to_string(), num_rows);
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    }
    #[test]
    fn test_2() {
        test("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    }
    #[test]
    fn test_3() {
        test("A", 10, "A");
    }
    #[test]
    fn test_4() {
        test("ABC", 1, "ABC");
    }
}

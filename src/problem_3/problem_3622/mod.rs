pub struct Solution;
impl Solution {
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    // why not u32
    pub fn check_divisibility(n: i32) -> bool {
        let s = n.to_string();
        let numbers = s.chars().filter_map(|c| c.to_digit(10));
        let divisor = numbers.clone().sum::<u32>() + numbers.product::<u32>();
        n % divisor as i32 == 0
    }
}
#[must_use]
// high memory (why?)
pub fn solution_1(n: i32) -> bool {
    let mut divisor = 1;
    let mut sum = 0;
    let mut product = 1;
    std::iter::from_fn(|| {
        let result = n / divisor;
        divisor *= 10;
        result.ne(&0).then_some(result % 10)
    })
    .for_each(|i| {
        sum += i;
        product *= i;
    });
    n % (sum + product) == 0
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(n: i32, expect: bool) {
        let solution = Solution::check_divisibility(n);
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(99, true);
    }
    #[test]
    fn test_2() {
        test(23, false);
    }
}

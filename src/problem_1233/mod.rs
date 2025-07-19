pub struct Solution;
impl Solution {
    #[must_use]
    pub fn remove_subfolders(mut folders: Vec<String>) -> Vec<String> {
        //folders.sort();
        folders.sort_unstable(); // I hope it more faster :D
        let mut now = folders[0].clone();
        folders.into_iter().fold(Vec::new(), |mut acc, folder| {
            if !is_parent(&now, &folder) {
                now.clone_from(&folder);
                acc.push(folder);
            }
            acc
        })
    }
}
/// too slow
#[must_use]
pub fn solution_1(mut folders: Vec<String>) -> Vec<String> {
    let mut checked: Vec<String> = Vec::new();
    while !folders.is_empty() {
        let parent = folders.swap_remove(0);
        folders.retain(|child| !is_parent(&parent, child));
        checked.retain(|child| !is_parent(&parent, child));
        checked.push(parent);
    }
    checked
}
fn is_parent(parent: &str, s: &str) -> bool {
    s.starts_with(&format!("{parent}/"))
}
#[cfg(test)]
mod tests {
    use super::Solution;
    fn test(input: &[&str], expect: &[&str]) {
        let folder: Vec<_> = input.iter().map(std::string::ToString::to_string).collect();
        let mut expect: Vec<_> = expect
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        expect.sort();
        let mut solution = Solution::remove_subfolders(folder);
        solution.sort();
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(
            &["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"],
            &["/a", "/c/d", "/c/f"],
        );
    }
    #[test]
    fn test_2() {
        test(&["/a", "/a/b/c", "/a/b/d"], &["/a"]);
    }
    #[test]
    fn test_3() {
        test(
            &["/a/b/c", "/a/b/ca", "/a/b/d"],
            &["/a/b/c", "/a/b/ca", "/a/b/d"],
        );
    }
}

pub struct Solution;
impl Solution {
    #[must_use]
    pub fn delete_duplicate_folder(mut _paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 写不来一点
        todo!()
    }
}
// #[cfg(test)]
// mod tests {
//     use super::Solution;
//     fn test(input: &[&[&str]], expect: &[&[&str]]) {
//         let folder: Vec<_> = input
//             .iter()
//             .map(|series| {
//                 series
//                     .iter()
//                     .map(std::string::ToString::to_string)
//                     .collect()
//             })
//             .collect();
//         let mut expect: Vec<Vec<_>> = expect
//             .iter()
//             .map(|series| {
//                 series
//                     .iter()
//                     .map(std::string::ToString::to_string)
//                     .collect()
//             })
//             .collect();
//         expect.sort();
//         let mut solution = Solution::delete_duplicate_folder(folder);
//         solution.sort();
//         assert_eq!(solution, expect);
//     }
//     #[test]
//     fn test_1() {
//         test(
//             &[
//                 &["a"],
//                 &["c"],
//                 &["d"],
//                 &["a", "b"],
//                 &["c", "b"],
//                 &["d", "a"],
//             ],
//             &[&["d"], &["d", "a"]],
//         );
//     }
//     #[test]
//     fn test_2() {
//         test(
//             &[
//                 &["a"],
//                 &["c"],
//                 &["a", "b"],
//                 &["c", "b"],
//                 &["a", "b", "x"],
//                 &["a", "b", "x", "y"],
//                 &["w"],
//                 &["w", "y"],
//             ],
//             &[&["c"], &["c", "b"], &["a"], &["a", "b"]],
//         );
//     }
//     #[test]
//     fn test_3() {
//         test(
//             &[&["a", "b"], &["c", "d"], &["c"], &["a"]],
//             &[&["c"], &["c", "d"], &["a"], &["a", "b"]],
//         );
//     }
//     #[test]
//     fn test_4() {
//         test(
//             &[
//                 &["a"],
//                 &["a", "x"],
//                 &["a", "x", "y"],
//                 &["a", "z"],
//                 &["b"],
//                 &["b", "x"],
//                 &["b", "x", "y"],
//                 &["b", "z"],
//             ],
//             &[],
//         );
//     }
//     #[test]
//     fn test_5() {
//         test(
//             &[
//                 &["a"],
//                 &["a", "x"],
//                 &["a", "x", "y"],
//                 &["a", "z"],
//                 &["b"],
//                 &["b", "x"],
//                 &["b", "x", "y"],
//                 &["b", "z"],
//                 &["b", "w"],
//             ],
//             &[&["b"], &["b", "w"], &["b", "z"], &["a"], &["a", "z"]],
//         );
//     }
// }

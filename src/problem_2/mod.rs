// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    #[must_use]
    // fast but use some memory
    pub fn add_two_numbers(
        mut list_1: Option<Box<ListNode>>,
        mut list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let should_carry = &mut false;
        let mut list = unchecked_add(list_1.as_deref(), list_2.as_deref(), should_carry);
        (list_1, list_2) = both_next(list_1, list_2);
        let mut current = &mut list.next;
        while !should_return(list_1.as_deref(), list_2.as_deref(), *should_carry) {
            *current = Some(Box::new(unchecked_add(
                list_1.as_deref(),
                list_2.as_deref(),
                should_carry,
            )));
            if let Some(next_list) = current.as_mut() {
                current = &mut next_list.next;
            } else {
                unreachable!()
            }
            (list_1, list_2) = both_next(list_1, list_2);
        }
        Some(Box::new(list))
    }
}
fn val(list: Option<&ListNode>) -> i32 {
    list.map(|list| list.val).unwrap_or_default()
}
fn both_next(
    list_1: Option<Box<ListNode>>,
    list_2: Option<Box<ListNode>>,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    (
        list_1.into_iter().find_map(|list| list.next),
        list_2.into_iter().find_map(|list| list.next),
    )
}
const fn both_none(list_1: Option<&ListNode>, list_2: Option<&ListNode>) -> bool {
    list_1.is_none() && list_2.is_none()
}
const fn should_return(
    list_1: Option<&ListNode>,
    list_2: Option<&ListNode>,
    should_carry: bool,
) -> bool {
    both_none(list_1, list_2) && !should_carry
}
fn unchecked_add(
    list_1: Option<&ListNode>,
    list_2: Option<&ListNode>,
    should_carry: &mut bool,
) -> ListNode {
    let result = val(list_1) + val(list_2) + i32::from(*should_carry);
    let val = if result >= 10 {
        *should_carry = true;
        result - 10
    } else {
        *should_carry = false;
        result
    };
    ListNode::new(val)
}
#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    fn build_node(series: &[i32]) -> ListNode {
        let mut list = ListNode::new(series[0]);
        let mut current = &mut list.next;
        for &i in series.iter().skip(1) {
            *current = Some(Box::new(ListNode::new(i)));
            current = &mut current.as_mut().unwrap().next;
        }
        *current = None;
        list
    }
    fn test(list_1: &[i32], list_2: &[i32], expect: &[i32]) {
        let list_1 = Some(Box::new(build_node(list_1)));
        let list_2 = Some(Box::new(build_node(list_2)));
        let expect = Some(Box::new(build_node(expect)));
        let solution = Solution::add_two_numbers(list_1, list_2);
        assert_eq!(solution, expect);
    }
    #[test]
    fn test_1() {
        test(&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]);
    }

    #[test]
    fn test_2() {
        test(&[0], &[0], &[0]);
    }
    #[test]
    fn test_3() {
        test(
            &[9, 9, 9, 9, 9, 9, 9],
            &[9, 9, 9, 9],
            &[8, 9, 9, 9, 0, 0, 0, 1],
        );
    }
}

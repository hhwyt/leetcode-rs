#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    fn some_box_list_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }

    #[test]
    fn test_empty_l1() {
        // l1: null
        // l2: 1
        // expected: 1
        let l1: Option<Box<ListNode>> = None;
        let l2 = tests::some_box_list_node(1, None);
        let expected = tests::some_box_list_node(1, None);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_empty_l2() {
        // l1: 1
        // l2: null
        // expected: 1
        let l1 = tests::some_box_list_node(9, None);
        let l2: Option<Box<ListNode>> = None;
        let expected = tests::some_box_list_node(9, None);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_no_carray() {
        // l1: 1 -> 2
        // l2: 2 -> 3
        // expected: 3 -> 5
        let l1 = tests::some_box_list_node(1, tests::some_box_list_node(2, None));
        let l2 = tests::some_box_list_node(2, tests::some_box_list_node(3, None));
        let expected = tests::some_box_list_node(3, tests::some_box_list_node(5, None));
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_general_carry() {
        // l1: 9 -> 1
        // l2 :2 -> 2
        // expected: 1 -> 4
        let l1 = tests::some_box_list_node(9, tests::some_box_list_node(1, None));
        let l2 = tests::some_box_list_node(2, tests::some_box_list_node(2, None));
        let expected = tests::some_box_list_node(1, tests::some_box_list_node(4, None));
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_end_in_carry() {
        // l1: 9 9
        // l2: 1 1
        // expected: 0 1 1
        let l1 = tests::some_box_list_node(9, tests::some_box_list_node(9, None));
        let l2 = tests::some_box_list_node(1, tests::some_box_list_node(1, None));
        let expected = tests::some_box_list_node(
            0,
            tests::some_box_list_node(1, tests::some_box_list_node(1, None)),
        );
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_not_equal_length() {
        // l1: 1
        // l2: 3 -> 2 -> 1
        // expected: 4 -> 2 -> 1
        let l1 = tests::some_box_list_node(1, None);
        let l2 = tests::some_box_list_node(
            3,
            tests::some_box_list_node(2, tests::some_box_list_node(1, None)),
        );
        let expected = tests::some_box_list_node(
            4,
            tests::some_box_list_node(2, tests::some_box_list_node(1, None)),
        );
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    fn add_two_numbers_recursive(
        l1: Option<&Box<ListNode>>,
        l2: Option<&Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2, carry) {
            (Some(l1), Some(l2), carry) => {
                let sum = l1.val + l2.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_two_numbers_recursive(
                        l1.next.as_ref(),
                        l2.next.as_ref(),
                        sum / 10,
                    ),
                }))
            }
            (Some(l), None, carry) | (None, Some(l), carry) => {
                let sum = l.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_two_numbers_recursive(l.next.as_ref(), None, sum / 10),
                }))
            }
            (None, None, 1) => Some(Box::new(ListNode::new(1))),
            _ => None,
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recursive(l1.as_ref(), l2.as_ref(), 0)
    }
}

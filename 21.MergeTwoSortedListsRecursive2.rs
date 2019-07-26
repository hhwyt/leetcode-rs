#![allow(dead_code)]

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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let some_box_list = |val, next| Some(Box::new(ListNode { val, next }));

        match (l1, l2) {
            (Some(node1), Some(node2)) => {
                if node1.val < node2.val {
                    some_box_list(node1.val, Self::merge_two_lists(node1.next, Some(node2)))
                } else {
                    some_box_list(node2.val, Self::merge_two_lists(Some(node1), node2.next))
                }
            }
            (Some(node1), None) => {
                some_box_list(node1.val, Self::merge_two_lists(node1.next, None))
            }
            (None, Some(node2)) => {
                some_box_list(node2.val, Self::merge_two_lists(node2.next, None))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Input: 1->2->4, 1->3->4
        // Output: 1->1->2->3->4->4
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let res = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(l1, l2), res);
    }
}

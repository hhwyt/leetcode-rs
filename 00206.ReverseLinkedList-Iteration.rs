#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut new_head: Option<Box<ListNode>> = None;
        while let Some(mut boxed_cur) = cur.take() {
            let next = boxed_cur.next.take();
            boxed_cur.next = new_head;
            new_head = Some(boxed_cur);
            cur = next;
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(v: Vec<Option<i32>>) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None;
        }

        let list_node_new = |val, next| {
            ListNode { val, next }
        };

        let mut next: Option<Box<ListNode>> = None;
        let mut head: Option<Box<ListNode>> = None;
        for i in (0..v.len()).rev() {
            head = Some(Box::new(list_node_new(v[i].unwrap(), next)));
            next = head.clone();
        }
        head
    }

    #[test]
    fn test_empty_list() {
        let input = list_from_vec(vec![]);
        let expected = list_from_vec(vec![]);
        assert_eq!(Solution::reverse_list(input), expected);
    }

    // Input: 1->2->3->4->5->NULL
    // Output: 5->4->3->2->1->NULL
    #[test]
    fn test_normal() {
        let input = list_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let expected = list_from_vec(vec![Some(5), Some(4), Some(3), Some(2), Some(1)]);
        assert_eq!(Solution::reverse_list(input), expected);
    }
}
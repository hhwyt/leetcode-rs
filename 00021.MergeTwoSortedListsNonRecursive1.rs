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
        let mut dummy = ListNode::new(0);
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut tail = &mut dummy;
        loop {
            if let Some(ref node1) = p1 {
                if let Some(ref node2) = p2 {
                    if node1.val <= node2.val {
                        tail.next = Some(Box::new(ListNode::new(node1.val)));
                        p1 = &node1.next;
                    } else {
                        tail.next = Some(Box::new(ListNode::new(node2.val)));
                        p2 = &node2.next;
                    }
                } else {
                    tail.next = Some(Box::new(ListNode::new(node1.val)));
                    p1 = &node1.next;
                }
            } else {
                if let Some(ref node2) = p2 {
                    tail.next = Some(Box::new(ListNode::new(node2.val)));
                    p2 = &node2.next;
                } else {
                    break;
                }
            }
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
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

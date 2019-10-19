struct Solution;

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
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut h = head;
        let mut p1 = h.as_mut().unwrap();
        while let Some(p2) = p1.next.as_mut() {
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h1 = None;
        let r1 = None;
        assert_eq!(Solution::delete_duplicates(h1), r1);

        let list_ctor = |val, next| Some(Box::new(ListNode { val, next }));

        let h2 = list_ctor(1, list_ctor(1, list_ctor(2, None)));
        let r2 = list_ctor(1, list_ctor(2, None));
        assert_eq!(Solution::delete_duplicates(h2), r2);

        let h3 = list_ctor(
            1,
            list_ctor(1, list_ctor(2, list_ctor(3, list_ctor(3, None)))),
        );
        let r3 = list_ctor(1, list_ctor(2, list_ctor(3, None)));
        assert_eq!(Solution::delete_duplicates(h3), r3);
    }
}

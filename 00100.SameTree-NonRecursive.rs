#![allow(dead_code)]
struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
        //    1         1
        //   / \       / \
        //  2   3     2   3
        let roota1 = Rc::new(RefCell::new(TreeNode::new(1)));
        roota1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        roota1.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let roota2 = Rc::new(RefCell::new(TreeNode::new(1)));
        roota2.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        roota2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert_eq!(Solution::is_same_tree(Some(roota1), Some(roota2)), true);

        //  1         1
        //  /           \
        // 2             2
        let rootb1 = Rc::new(RefCell::new(TreeNode::new(1)));
        rootb1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let rootb2 = Rc::new(RefCell::new(TreeNode::new(1)));
        rootb2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(Solution::is_same_tree(Some(rootb1), Some(rootb2)), false);
    }
}

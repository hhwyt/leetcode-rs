use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn inorder_traversal(mut some_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        while let Some(rc_root) = some_root {
            let root = rc_root.borrow();
            match root.left.clone() {
                Some(mut cur) => {
                    while cur.borrow().right.is_some() && !Rc::ptr_eq(&cur.borrow().right.clone().unwrap(), &rc_root) {
                        cur = cur.clone().borrow().right.clone().unwrap();
                    }

                    if cur.borrow().right.is_none() {
                        cur.borrow_mut().right = Some(rc_root.clone());
                        some_root = root.left.clone();
                    } else {
                        cur.borrow_mut().right = None;
                        res.push(root.val);
                        some_root = root.right.clone();
                    }
                }
                None => {
                    res.push(root.val);
                    some_root = root.right.clone();
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_2_tree_recursive(v: &Vec<Option<i32>>, cur: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if v.get(cur).is_none() || v.get(cur).unwrap().is_none() {
            return None;
        }

        // left_child = parent * 2 + 1
        let mut root = TreeNode::new(v.get(cur).unwrap().unwrap());
        let left_child = cur * 2 + 1;
        root.left = tests::vec_2_tree_recursive(v, left_child);
        root.right = tests::vec_2_tree_recursive(v, left_child + 1);
        Some(Rc::new(RefCell::new(root)))
    }

    fn vec_2_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        return tests::vec_2_tree_recursive(v.as_ref(), 0);
    }

    #[test]
    fn test_normal() {
        /*
             1
            / \
           2   3
        */
        let v = vec![Some(1), Some(2), Some(3)];
        let root = tests::vec_2_tree(v);
        let expected = vec![2, 1, 3];
        assert_eq!(Solution::inorder_traversal(root), expected);
    }

    #[test]
    fn test_empty_tree() {
        let v = vec![];
        let root = tests::vec_2_tree(v);
        let expected: Vec<i32> = vec![];
        assert_eq!(Solution::inorder_traversal(root), expected);
    }
}
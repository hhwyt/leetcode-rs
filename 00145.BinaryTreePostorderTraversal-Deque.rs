use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut res = Vec::new();
        while !queue.is_empty() {
            match queue.pop_back().unwrap() {
                Some(root) => {
                    let mut root = (*root).borrow_mut();
                    res.push(root.val);

                    let left = root.left.take();
                    if left.is_some() {
                        queue.push_back(left);
                    }

                    let right = root.right.take();
                    if right.is_some() {
                        queue.push_back(right);
                    }
                }
                None => {}
            }
        }
        res.reverse();
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
        let expected = vec![2, 3, 1];
        assert_eq!(Solution::postorder_traversal(root), expected);
    }

    #[test]
    fn test_empty_tree() {
        let v = vec![];
        let root = tests::vec_2_tree(v);
        let expected: Vec<i32> = vec![];
        assert_eq!(Solution::postorder_traversal(root), expected);
    }
}
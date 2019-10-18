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

#[derive(Eq, PartialEq)]
enum Operation {
    Visit,
    NotVisit,
}

struct Guide {
    node: Option<Rc<RefCell<TreeNode>>>,
    op: Operation,
}

impl Guide {
    fn new(node: Option<Rc<RefCell<TreeNode>>>, op: Operation) -> Guide {
        Guide {
            node,
            op,
        }
    }
}

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(Guide::new(root.clone(), Operation::NotVisit));

        while !queue.is_empty() {
            let guide = queue.pop_back().unwrap();
            let (op, some_node) = (guide.op, guide.node);
            match some_node {
                Some(ref node) => {
                    if op == Operation::Visit {
                        res.push(node.borrow().val);
                    } else {
                        queue.push_back(Guide::new(node.borrow().right.clone(), Operation::NotVisit));
                        queue.push_back(Guide::new(some_node.clone(), Operation::Visit));
                        queue.push_back(Guide::new(node.borrow().left.clone(), Operation::NotVisit));
                    }
                }
                None => {
                    continue;
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
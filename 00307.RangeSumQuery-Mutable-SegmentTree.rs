use std::rc::Rc;
use std::cell::RefCell;

struct SegmentTreeNode {
    start: usize,
    end: usize,
    sum: i32,
    left: Option<Rc<RefCell<SegmentTreeNode>>>,
    right: Option<Rc<RefCell<SegmentTreeNode>>>,
}

impl SegmentTreeNode {
    fn new(start: usize, end: usize, sum: i32) -> SegmentTreeNode {
        Self::construct(start, end, sum, None, None)
    }

    fn with_child(start: usize, end: usize, sum: i32,
                  left: Option<Rc<RefCell<SegmentTreeNode>>>,
                  right: Option<Rc<RefCell<SegmentTreeNode>>>) -> SegmentTreeNode {
        Self::construct(start, end, sum, left, right)
    }

    fn construct(start: usize, end: usize, sum: i32,
                 left: Option<Rc<RefCell<SegmentTreeNode>>>,
                 right: Option<Rc<RefCell<SegmentTreeNode>>>)
                 -> SegmentTreeNode {
        SegmentTreeNode {
            start,
            end,
            sum,
            left,
            right,
        }
    }
}

struct NumArray {
    root: Option<Rc<RefCell<SegmentTreeNode>>>,
    len: usize,
}

impl NumArray {
    fn segment_tree(nums: &Vec<i32>, start: usize, end: usize) -> Rc<RefCell<SegmentTreeNode>> {
        if start == end {
            return Rc::new(RefCell::new(SegmentTreeNode::new(start, end, nums[start])));
        }

        let mid = start + (end - start) / 2;
        let rc_left = Self::segment_tree(nums, start, mid);
        let rc_right = Self::segment_tree(nums, mid + 1, end);
        let sum = rc_left.borrow().sum + rc_right.borrow().sum;
        Rc::new(RefCell::new(SegmentTreeNode::with_child(start, end, sum, Some(rc_left), Some(rc_right))))
    }

    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            root: if nums.is_empty() { None } else { Some(Self::segment_tree(&nums, 0, nums.len() - 1)) },
            len: nums.len(),
        }
    }

    fn update_segment_tree(some_root: &mut Option<Rc<RefCell<SegmentTreeNode>>>, i: usize, val: i32) {
        match some_root {
            Some(rc_root) => {
                if i == rc_root.borrow().start && i == rc_root.borrow().end {
                    rc_root.borrow_mut().sum = val;
                    return;
                }
                let mid = rc_root.borrow().start + (rc_root.borrow().end - rc_root.borrow().start) / 2;
                if i <= mid {
                    Self::update_segment_tree(&mut rc_root.borrow_mut().left, i, val);
                } else {
                    Self::update_segment_tree(&mut rc_root.borrow_mut().right, i, val);
                }
                let left_sum = rc_root.borrow().left.as_ref().map(|root| root.borrow().sum).unwrap();
                let right_sum = rc_root.borrow().right.as_ref().map(|root| root.borrow().sum).unwrap();
                rc_root.borrow_mut().sum = left_sum + right_sum;
            }
            None => {}
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        if self.root.is_none() || i < 0 || i >= self.len() as i32 {
            return;
        }

        Self::update_segment_tree(&mut self.root, i as usize, val)
    }

    fn range_query(some_root: &Option<Rc<RefCell<SegmentTreeNode>>>, i: usize, j: usize) -> i32 {
        match some_root {
            Some(rc_root) => {
                if i == rc_root.borrow().start && i == rc_root.borrow().end {
                    return rc_root.borrow().sum;
                }
                let mid = rc_root.borrow().start + (rc_root.borrow().end - rc_root.borrow().start) / 2;
                if mid >= j {
                    Self::range_query(&rc_root.borrow().left, i, j)
                } else if mid + 1 <= i {
                    Self::range_query(&rc_root.borrow().right, i, j)
                } else {
                    Self::range_query(&rc_root.borrow().left, i, mid) +
                        Self::range_query(&rc_root.borrow().right, mid + 1, j)
                }
            }
            None => {
                0
            }
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        if i > j || i < 0 || j >= self.len() as i32 {
            return 0;
        }

        Self::range_query(&self.root, i as usize, j as usize)
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        let nums = vec![1, 3, 5];
        let mut num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 2), 9);
        num_array.update(1, 2);
        assert_eq!(num_array.sum_range(0, 2), 8);
    }

    #[test]
    fn test_empty() {
        let nums = vec![];
        let mut num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 2), 0);
        num_array.update(1, 2);
    }

    #[test]
    fn test_i_grater_than_j() {
        let nums = vec![1, 2];
        let num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(2, 0), 0);
    }
}
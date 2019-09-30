#![allow(dead_code)]

use std::ptr;
use std::mem;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug)]
struct LFUEntry {
    key: i32,
    val: i32,

    prev: *mut LFUEntry,
    next: *mut LFUEntry,

    parent: *mut FreqList,
}

impl LFUEntry {
    fn new(key: i32, val: i32) -> LFUEntry {
        LFUEntry {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            parent: ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
struct FreqList {
    cnt: usize,

    prev: *mut FreqList,
    next: *mut FreqList,

    head: *mut LFUEntry,
    tail: *mut LFUEntry,
}

impl FreqList {
    fn new(cnt: usize) -> FreqList {
        let list = FreqList {
            cnt,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),

            head: Box::into_raw(Box::new(unsafe { mem::uninitialized::<LFUEntry>() })),
            tail: Box::into_raw(Box::new(unsafe { mem::uninitialized::<LFUEntry>() })),

        };

        unsafe {
            (*list.head).next = list.tail;
            (*list.tail).prev = list.head;
        }

        list
    }

    fn detach(&mut self, entry: *mut LFUEntry) {
        unsafe {
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
        }
    }

    fn attach(&mut self, entry: *mut LFUEntry) {
        unsafe {
            (*entry).next = (*self.head).next;
            (*(*entry).next).prev = entry;
            (*self.head).next = entry;
            (*entry).prev = self.head;
        }
    }
}

#[derive(Debug)]
struct LFUCache {
    map: HashMap<i32, Box<LFUEntry>>,
    cap: usize,

    head: *mut FreqList,
    tail: *mut FreqList,
}

impl LFUCache {
    pub fn new(cap: i32) -> Self {
        let cache = LFUCache {
            map: HashMap::default(),
            cap: cap as usize,
            head: Box::into_raw(Box::new(FreqList::new(0))),
            tail: Box::into_raw(Box::new(FreqList::new(0))),
        };

        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }

        match self.map.get_mut(&key) {
            Some(entry) => {
                entry.val = value;
                let entry_ptr: *mut LFUEntry = &mut **entry;
                self.upgrade(entry_ptr);
            }
            None => {
                let mut entry = if self.cap == self.len() {
                    let mut entry = self.evict();
                    entry.key = key;
                    entry.val = value;
                    entry
                } else {
                    Box::new(LFUEntry::new(key, value))
                };

                let first_list = unsafe {
                    if (*(*self.head).next).cnt == 1 {
                        (*self.head).next
                    } else {
                        let list = Box::into_raw(Box::new(FreqList::new(1)));
                        self.attach(list);
                        list
                    }
                };
                entry.parent = first_list;

                let entry_ptr = &mut *entry as *mut LFUEntry;
                unsafe { (*first_list).attach(entry_ptr); }

                self.map.insert(key, entry);
            }
        };
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get_mut(&key) {
            Some(entry) => {
                let entry_ptr = &mut **entry as *mut LFUEntry;
                self.upgrade(entry_ptr);
                unsafe { (*entry_ptr).val }
            }
            None => {
                -1
            }
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    fn detach(&mut self, list: *mut FreqList) {
        unsafe {
            (*(*list).prev).next = (*list).next;
            (*(*list).next).prev = (*list).prev;
        }
    }

    fn detach_if_empty(&mut self, list: *mut FreqList) {
        unsafe {
            if (*(*list).head).next == (*list).tail {
                self.detach(list);
            }
        }
    }

    fn attach(&mut self, list: *mut FreqList) {
        self.insert_after(self.head, list);
    }

    fn insert_before(&mut self, target: *mut FreqList, list: *mut FreqList) {
        unsafe {
            self.insert_after((*target).prev, list);
        }
    }

    fn insert_after(&mut self, target: *mut FreqList, list: *mut FreqList) {
        unsafe {
            (*list).next = (*target).next;
            (*(*list).next).prev = list;
            (*target).next = list;
            (*list).prev = target;
        }
    }

    fn upgrade(&mut self, entry: *mut LFUEntry) {
        unsafe {
            let old_list = (*entry).parent;
            let next_list = (*old_list).next;

            (*old_list).detach(entry);
            self.detach_if_empty(old_list);

            let cnt = (*old_list).cnt;
            let new_list = if (*next_list).cnt == cnt + 1 {
                next_list
            } else {
                let list = Box::into_raw(Box::new(FreqList::new(cnt + 1)));
                self.insert_before(next_list, list);
                list
            };
            (*new_list).attach(entry);
            (*entry).parent = new_list;
        }
    }

    fn evict(&mut self) -> Box<LFUEntry> {
        let first_list = unsafe { (*self.head).next };
        let old_key = unsafe { (*(*(*first_list).tail).prev).key };
        let mut entry = self.map.remove(&old_key).unwrap();
        let entry_ptr = &mut *entry as *mut LFUEntry;
        unsafe {
            (*first_list).detach(entry_ptr);
            self.detach_if_empty(first_list);
        }
        entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_series_operations(operations: Vec<&str>, operation_nums: Vec<Vec<i32>>) {
        let mut cache = LFUCache::new(operation_nums[0][0]); // Not robust
        for (i, &op) in operations.iter().enumerate() {
            match op {
                "put" => {
                    cache.put(operation_nums[i][0], operation_nums[i][1]);
                }
                "get" => {
                    cache.get(operation_nums[i][0]);
                }
                _ => {}
            }
        }
    }

    fn test_series_operations_with_results(operations: Vec<&str>, operation_nums: Vec<Vec<i32>>, expected_results: Vec<Option<i32>>) {
        let mut cache = LFUCache::new(operation_nums[0][0]); // Not robust
        for (i, &op) in operations.iter().enumerate() {
            match op {
                "put" => {
                    cache.put(operation_nums[i][0], operation_nums[i][1]);
                }
                "get" => {
                    assert_eq!(cache.get(operation_nums[i][0]), expected_results[i].unwrap())
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_cap_is_zero() {
        let operations = vec!["LFUCache", "put", "get"];
        let operation_nums = vec![vec![0], vec![0, 0], vec![0]];
        let expected_results = vec![None, None, Some(-1)];
        test_series_operations_with_results(operations, operation_nums, expected_results);
    }

    #[test]
    fn test_1() {
        let mut cache = LFUCache::new(2);
        assert_eq!(cache.len(), 0);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(1), 1);

        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);

        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);

        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_2() {
        let operations = vec!["LFUCache", "put", "put", "get", "get", "get", "put", "put", "get", "get", "get", "get"];
        let operations_nums = vec![vec![3], vec![2, 2], vec![1, 1], vec![2], vec![1], vec![2], vec![3, 3], vec![4, 4], vec![3], vec![2], vec![1], vec![4]];
        let expected_results = vec![None, None, None, Some(2), Some(1), Some(2), None, None, Some(-1), Some(2), Some(1), Some(4)];
        test_series_operations_with_results(operations, operations_nums, expected_results);
    }

    #[test]
    fn test_3() {
        let operations = vec!["LFUCache", "put", "get", "put", "get", "get"];
        let operation_nums = vec![vec![1], vec![2, 1], vec![2], vec![3, 2], vec![2], vec![3]];
        let expected_results = vec![None, None, Some(1), None, Some(-1), Some(2)];
        test_series_operations_with_results(operations, operation_nums, expected_results);
    }

    #[test]
    fn test_4() {
        let operations = vec!["LFUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get", "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put"];
        let operation_nums = vec![vec![10], vec![10, 13], vec![3, 17], vec![6, 11], vec![10, 5], vec![9, 10], vec![13], vec![2, 19], vec![2], vec![3], vec![5, 25], vec![8], vec![9, 22], vec![5, 5], vec![1, 30], vec![11], vec![9, 12], vec![7], vec![5], vec![8], vec![9], vec![4, 30], vec![9, 3], vec![9], vec![10], vec![10], vec![6, 14], vec![3, 1], vec![3], vec![10, 11], vec![8], vec![2, 14], vec![1], vec![5], vec![4], vec![11, 4], vec![12, 24], vec![5, 18], vec![13], vec![7, 23], vec![8], vec![12], vec![3, 27], vec![2, 12], vec![5], vec![2, 9], vec![13, 4], vec![8, 18], vec![1, 7], vec![6], vec![9, 29], vec![8, 21], vec![5], vec![6, 30], vec![1, 12], vec![10], vec![4, 15], vec![7, 22], vec![11, 26], vec![8, 17], vec![9, 29], vec![5], vec![3, 4], vec![11, 30], vec![12], vec![4, 29], vec![3], vec![9], vec![6], vec![3, 4], vec![1], vec![10], vec![3, 29], vec![10, 28], vec![1, 20], vec![11, 13], vec![3], vec![3, 12], vec![3, 8], vec![10, 9], vec![3, 26], vec![8], vec![7], vec![5], vec![13, 17], vec![2, 27], vec![11, 15], vec![12], vec![9, 19], vec![2, 15], vec![3, 16], vec![1], vec![12, 17], vec![9, 1], vec![6, 19], vec![4], vec![5], vec![5], vec![8, 1], vec![11, 7], vec![5, 2], vec![9, 28], vec![1], vec![2, 2], vec![7, 4], vec![4, 22], vec![7, 24], vec![9, 26], vec![13, 28], vec![11, 26]];
        let expected_results = vec![None, None, None, None, None, None, Some(-1), None, Some(19), Some(17), None, Some(-1), None, None, None, Some(-1), None, Some(-1), Some(5), Some(-1), Some(12), None, None, Some(3), Some(5), Some(5), None, None, Some(1), None, Some(-1), None, Some(30), Some(5), Some(30), None, None, None, Some(-1), None, Some(-1), Some(24), None, None, Some(18), None, None, None, None, Some(14), None, None, Some(18), None, None, Some(11), None, None, None, None, None, Some(18), None, None, Some(-1), None, Some(4), Some(29), Some(30), None, Some(12), Some(11), None, None, None, None, Some(29), None, None, None, None, Some(17), Some(-1), Some(18), None, None, None, Some(-1), None, None, None, Some(20), None, None, None, Some(29), Some(18), Some(18), None, None, None, None, Some(20), None, None, None, None, None, None, None];
        test_series_operations_with_results(operations, operation_nums, expected_results);
    }
}

use std::collections::HashMap;
use std::mem;
use std::ptr;

#[derive(Debug, Clone)]
struct LRUEntry {
    key: i32,
    val: i32,
    prev: *mut LRUEntry,
    next: *mut LRUEntry,
}

#[derive(Debug, Clone)]
struct LRUCache {
    map: HashMap<i32, Box<LRUEntry>>,
    cap: usize,
    head: *mut LRUEntry,
    tail: *mut LRUEntry,
}

impl LRUEntry {
    pub fn new(key: i32, val: i32) -> LRUEntry {
        LRUEntry {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cache = LRUCache {
            map: HashMap::with_capacity(capacity as usize),
            cap: capacity as usize,
            head: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
            tail: unsafe { Box::into_raw(Box::new(mem::uninitialized::<LRUEntry>())) },
        };

        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get_mut(&key) {
            Some(node) => {
                let node_ptr = &mut **node as *mut LRUEntry;

                self.detach(node_ptr);
                self.attach(node_ptr);

                unsafe { (*node_ptr).val }
            }
            None => -1
        }
    }


    pub fn put(&mut self, key: i32, value: i32) {
        match self.map.get_mut(&key) {
            Some(node) => {
                let node_ptr = &mut **node as *mut LRUEntry;
                unsafe {
                    (*node_ptr).val = value;
                }
                self.detach(node_ptr);
                self.attach(node_ptr);
            }
            None => {
                let mut node = if self.len() == self.cap {
                    let old_key = unsafe { (*(*self.tail).prev).key };
                    let mut old_node = self.map.remove(&old_key).unwrap();

                    old_node.key = key;
                    old_node.val = value;

                    let node_ptr = &mut *old_node as *mut LRUEntry;
                    self.detach(node_ptr);

                    old_node
                } else {
                    Box::new(LRUEntry::new(key, value))
                };

                let node_ptr = &mut *node as *mut LRUEntry;
                self.attach(node_ptr);

                self.map.insert(key, node);
            }
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn detach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    fn attach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*(*self.head).next).prev = node;
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_series_operations(operations: Vec<&str>, operation_nums: Vec<Vec<i32>>) {
        let mut cache = LRUCache::new(operation_nums[0][0]); // Not robust
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
        let mut cache = LRUCache::new(operation_nums[0][0]); // Not robust
        for (i, &op) in operations.iter().enumerate() {
            // println!("action: {:?}, cache: {:?}", operation_nums[i], cache);
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
    fn test_0() {
        let operations = vec!["LRUCache", "put", "put", "put", "get", "put", "get", "get"];
        let operation_nums = vec![vec![2], vec![1, 1], vec![2, 2], vec![3, 3], vec![1], vec![4, 4], vec![2], vec![3]];
        let operation_results = vec![None, None, None, None, Some(-1), None, Some(-1), Some(3)];
        tests::test_series_operations_with_results(operations, operation_nums, operation_results);
    }

    #[test]
    fn test_1() {
        let operations = vec!["LRUCache", "put", "put", "get", "put", "get", "put", "get"];
        let operation_nums = vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![4, 4], vec![1]];
        let operation_results = vec![None, None, None, Some(1), None, Some(-1), None, Some(-1)];
        tests::test_series_operations_with_results(operations, operation_nums, operation_results);
    }

    #[test]
    fn test_2() {
        let operations = vec!["LRUCache", "put", "get", "put", "get", "get"];
        let operation_nums = vec![vec![1], vec![2, 1], vec![2], vec![3, 2], vec![2], vec![3]];
        let operation_results = vec![None, None, Some(1), None, Some(-1), Some(2)];
        tests::test_series_operations_with_results(operations, operation_nums, operation_results);
    }

    #[test]
    fn test_3() {
        let operations = vec!["LRUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get", "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put"];
        let operation_nums = vec![vec![10], vec![10, 13], vec![3, 17], vec![6, 11], vec![10, 5], vec![9, 10], vec![13], vec![2, 19], vec![2], vec![3], vec![5, 25], vec![8], vec![9, 22], vec![5, 5], vec![1, 30], vec![11], vec![9, 12], vec![7], vec![5], vec![8], vec![9], vec![4, 30], vec![9, 3], vec![9], vec![10], vec![10], vec![6, 14], vec![3, 1], vec![3], vec![10, 11], vec![8], vec![2, 14], vec![1], vec![5], vec![4], vec![11, 4], vec![12, 24], vec![5, 18], vec![13], vec![7, 23], vec![8], vec![12], vec![3, 27], vec![2, 12], vec![5], vec![2, 9], vec![13, 4], vec![8, 18], vec![1, 7], vec![6], vec![9, 29], vec![8, 21], vec![5], vec![6, 30], vec![1, 12], vec![10], vec![4, 15], vec![7, 22], vec![11, 26], vec![8, 17], vec![9, 29], vec![5], vec![3, 4], vec![11, 30], vec![12], vec![4, 29], vec![3], vec![9], vec![6], vec![3, 4], vec![1], vec![10], vec![3, 29], vec![10, 28], vec![1, 20], vec![11, 13], vec![3], vec![3, 12], vec![3, 8], vec![10, 9], vec![3, 26], vec![8], vec![7], vec![5], vec![13, 17], vec![2, 27], vec![11, 15], vec![12], vec![9, 19], vec![2, 15], vec![3, 16], vec![1], vec![12, 17], vec![9, 1], vec![6, 19], vec![4], vec![5], vec![5], vec![8, 1], vec![11, 7], vec![5, 2], vec![9, 28], vec![1], vec![2, 2], vec![7, 4], vec![4, 22], vec![7, 24], vec![9, 26], vec![13, 28], vec![11, 26]];
        let operation_results = vec![None, None, None, None, None, None, Some(-1), None, Some(19), Some(17), None, Some(-1), None, None, None, Some(-1), None, Some(-1), Some(5), Some(-1), Some(12), None, None, Some(3), Some(5), Some(5), None, None, Some(1), None, Some(-1), None, Some(30), Some(5), Some(30), None, None, None, Some(-1), None, Some(-1), Some(24), None, None, Some(18), None, None, None, None, Some(-1), None, None, Some(18), None, None, Some(-1), None, None, None, None, None, Some(18), None, None, Some(-1), None, Some(4), Some(29), Some(30), None, Some(12), Some(-1), None, None, None, None, Some(29), None, None, None, None, Some(17), Some(22), Some(18), None, None, None, Some(-1), None, None, None, Some(20), None, None, None, Some(-1), Some(18), Some(18), None, None, None, None, Some(20), None, None, None, None, None, None, None];
        tests::test_series_operations_with_results(operations, operation_nums, operation_results);
    }

    #[test]
    fn test_key_already_exist() {
        let operations = vec!["LRUCache", "get", "put", "get", "put", "put", "get", "get"];
        let operation_nums = vec![vec![2], vec![2], vec![2, 6], vec![1], vec![1, 5], vec![1, 2], vec![1], vec![2]];
        let expected_results = vec![None, Some(-1), None, Some(-1), None, None, Some(2), Some(6)];
        self::test_series_operations_with_results(operations, operation_nums, expected_results);
    }

    #[test]
    fn test_key_already_exist_2() {
        let operations = vec!["LRUCache", "put", "put", "put", "put", "get", "get"];
        let operation_nums = vec![vec![2], vec![2, 1], vec![1, 1], vec![2, 3], vec![4, 1], vec![1], vec![2]];
        let expected_results = vec![None, None, None, None, None, Some(-1), Some(3)];
        self::test_series_operations_with_results(operations, operation_nums, expected_results);
    }

    #[test]
    fn test_time_limit() {
        // This test case from LeetCode is too long to build test results.
        let operations = vec!["LRUCache", "put", "get", "put", "get", "get", "get", "put", "get", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "put", "get", "put", "put", "get", "put", "put", "put", "get", "get", "put", "put", "get", "put", "get", "get", "get", "get", "put", "get", "put", "put", "put", "get", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "get", "get", "get", "put", "get", "put", "get", "put", "put", "get", "get", "get", "get", "get", "get", "put", "put", "get", "get", "put", "get", "put", "put", "get", "get", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "get", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "get", "get", "get", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "get", "put", "get", "put", "get", "get", "get", "get", "put", "get", "get", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "get", "get", "put", "get", "get", "get", "get", "put", "get", "put", "put", "get", "get", "put", "put", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "put", "get", "put", "get", "put", "get", "put", "get", "put", "get", "put", "put", "get", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get", "put", "get", "put", "put", "put", "put", "get", "get", "put", "get", "put", "get", "put", "get", "put", "get", "get", "get", "get", "get", "put", "put", "get", "put", "get", "put", "put", "get", "get", "put", "get", "get", "put", "get", "put", "put", "get", "put", "put", "get", "get", "put", "put", "get", "get", "put", "get", "put", "put", "get", "put", "put", "put", "get", "put", "put", "get", "get", "get", "get", "put", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "get", "put", "get", "get", "put", "get", "put", "put", "get", "get", "put", "get", "put", "put", "get", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "get", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "get", "put", "put", "get", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "put", "get", "get", "put", "get", "put", "put", "get", "get", "get", "get", "get", "put", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "get", "put", "put", "put", "get", "get", "get", "get", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "get", "get", "put", "get", "get", "get", "get", "get", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "get", "get", "get", "get", "put", "put", "put", "get", "get", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "get", "put", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "put", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "get", "get", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "get", "put", "get", "get", "put", "put", "put", "get", "get", "put", "put", "get", "get", "get", "put", "get", "get", "get", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "get", "get", "put", "get", "put", "put", "get", "get", "put", "get", "get", "get", "get", "put", "put", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "get", "put", "get", "get", "get", "put", "put", "get", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "put", "get", "put", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "get", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "get", "get", "get", "put", "put", "get", "put", "get", "get", "get", "put", "put", "put", "put", "put", "put", "put", "get", "get", "put", "put", "put", "put", "put", "get", "get", "put", "put", "get", "put", "get", "put", "put", "put", "get", "put", "get", "put", "get", "get", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "get", "put", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "get", "put", "put", "put", "put", "get", "put", "get", "put", "put", "put", "put", "put", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put", "get", "put", "get", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "put", "get", "put", "get", "put", "get", "put", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "get", "put", "put", "put", "get", "put", "get", "put", "put", "get", "put", "get", "put", "get", "put", "get", "get", "put", "get", "get", "put", "put", "put", "get", "put", "put", "get", "get", "put", "get", "put", "get", "put", "put", "get", "get"];
        let operation_nums = vec![vec![105], vec![33, 219], vec![39], vec![96, 56], vec![129], vec![115], vec![112], vec![3, 280], vec![40], vec![85, 193], vec![10, 10], vec![100, 136], vec![12, 66], vec![81, 261], vec![33, 58], vec![3], vec![121, 308], vec![129, 263], vec![105], vec![104, 38], vec![65, 85], vec![3, 141], vec![29, 30], vec![80, 191], vec![52, 191], vec![8, 300], vec![136], vec![48, 261], vec![3, 193], vec![133, 193], vec![60, 183], vec![128, 148], vec![52, 176], vec![48], vec![48, 119], vec![10, 241], vec![124], vec![130, 127], vec![61], vec![124, 27], vec![94], vec![29, 304], vec![102, 314], vec![110], vec![23, 49], vec![134, 12], vec![55, 90], vec![14], vec![104], vec![77, 165], vec![60, 160], vec![117], vec![58, 30], vec![54], vec![136], vec![128], vec![131], vec![48, 114], vec![136], vec![46, 51], vec![129, 291], vec![96, 207], vec![131], vec![89, 153], vec![120, 154], vec![111], vec![47], vec![5], vec![114, 157], vec![57, 82], vec![113, 106], vec![74, 208], vec![56], vec![59], vec![100], vec![132], vec![127, 202], vec![75], vec![102, 147], vec![37], vec![53, 79], vec![119, 220], vec![47], vec![101], vec![89], vec![20], vec![93], vec![7], vec![48, 109], vec![71, 146], vec![43], vec![122], vec![3, 160], vec![17], vec![80, 22], vec![80, 272], vec![75], vec![117], vec![76, 204], vec![74, 141], vec![107, 93], vec![34, 280], vec![31, 94], vec![132], vec![71, 258], vec![61], vec![60], vec![69, 272], vec![46], vec![42, 264], vec![87, 126], vec![107, 236], vec![131, 218], vec![79], vec![41, 71], vec![94, 111], vec![19, 124], vec![52, 70], vec![131], vec![103], vec![81], vec![126], vec![61, 279], vec![37, 100], vec![95], vec![54], vec![59, 136], vec![101, 219], vec![15, 248], vec![37, 91], vec![11, 174], vec![99, 65], vec![105, 249], vec![85], vec![108, 287], vec![96, 4], vec![70], vec![24], vec![52, 206], vec![59, 306], vec![18, 296], vec![79, 95], vec![50, 131], vec![3, 161], vec![2, 229], vec![39, 183], vec![90, 225], vec![75, 23], vec![136, 280], vec![119], vec![81, 272], vec![106], vec![106], vec![70], vec![73, 60], vec![19, 250], vec![82, 291], vec![117, 53], vec![16, 176], vec![40], vec![7, 70], vec![135, 212], vec![59], vec![81, 201], vec![75, 305], vec![101], vec![8, 250], vec![38], vec![28, 220], vec![21], vec![105, 266], vec![105], vec![85], vec![55], vec![6], vec![78, 83], vec![126], vec![102], vec![66], vec![61, 42], vec![127, 35], vec![117, 105], vec![128], vec![102], vec![50], vec![24, 133], vec![40, 178], vec![78, 157], vec![71, 22], vec![25], vec![82], vec![129], vec![126, 12], vec![45], vec![40], vec![86], vec![100], vec![30, 110], vec![49], vec![47, 185], vec![123, 101], vec![102], vec![5], vec![40, 267], vec![48, 155], vec![108], vec![45], vec![14, 182], vec![20, 117], vec![43, 124], vec![38], vec![77, 158], vec![111], vec![39], vec![69, 126], vec![113, 199], vec![21, 216], vec![11], vec![117, 207], vec![30], vec![97, 84], vec![109], vec![99, 218], vec![109], vec![113, 1], vec![62], vec![49, 89], vec![53, 311], vec![126], vec![32, 153], vec![14, 296], vec![22], vec![14, 225], vec![49], vec![75], vec![61, 241], vec![7], vec![6], vec![31], vec![75, 15], vec![115], vec![84, 181], vec![125, 111], vec![105, 94], vec![48, 294], vec![106], vec![61], vec![53, 190], vec![16], vec![12, 252], vec![28], vec![111, 122], vec![122], vec![10, 21], vec![59], vec![72], vec![39], vec![6], vec![126], vec![131, 177], vec![105, 253], vec![26], vec![43, 311], vec![79], vec![91, 32], vec![7, 141], vec![38], vec![13], vec![79, 135], vec![43], vec![94], vec![80, 182], vec![53], vec![120, 309], vec![3, 109], vec![97], vec![9, 128], vec![114, 121], vec![56], vec![56], vec![124, 86], vec![34, 145], vec![131], vec![78], vec![86, 21], vec![98], vec![115, 164], vec![47, 225], vec![95], vec![89, 55], vec![26, 134], vec![8, 15], vec![11], vec![84, 276], vec![81, 67], vec![46], vec![39], vec![92], vec![96], vec![89, 51], vec![136, 240], vec![45], vec![27], vec![24, 209], vec![82, 145], vec![10], vec![104, 225], vec![120, 203], vec![121, 108], vec![11, 47], vec![89], vec![80, 66], vec![16], vec![95, 101], vec![49], vec![1], vec![77, 184], vec![27], vec![74, 313], vec![14, 118], vec![16], vec![74], vec![88, 251], vec![124], vec![58, 101], vec![42, 81], vec![2], vec![133, 101], vec![16], vec![1, 254], vec![25, 167], vec![53, 56], vec![73, 198], vec![48], vec![30], vec![95], vec![90, 102], vec![92, 56], vec![2, 130], vec![52, 11], vec![9], vec![23], vec![53, 275], vec![23, 258], vec![57], vec![136, 183], vec![75, 265], vec![85], vec![68, 274], vec![15, 255], vec![85], vec![33, 314], vec![101, 223], vec![39, 248], vec![18, 261], vec![37, 160], vec![112], vec![65], vec![31, 240], vec![40, 295], vec![99, 231], vec![123], vec![34, 43], vec![87], vec![80], vec![47, 279], vec![89, 299], vec![72], vec![26, 277], vec![92, 13], vec![46, 92], vec![67, 163], vec![85, 184], vec![38], vec![35, 65], vec![70], vec![81], vec![40, 65], vec![80], vec![80, 23], vec![76, 258], vec![69], vec![133], vec![123, 196], vec![119, 212], vec![13, 150], vec![22, 52], vec![20, 105], vec![61, 233], vec![97], vec![128, 307], vec![85], vec![80], vec![73], vec![30], vec![46, 44], vec![95], vec![121, 211], vec![48, 307], vec![2], vec![27, 166], vec![50], vec![75, 41], vec![101, 105], vec![2], vec![110, 121], vec![32, 88], vec![75, 84], vec![30, 165], vec![41, 142], vec![128, 102], vec![105, 90], vec![86, 68], vec![13, 292], vec![83, 63], vec![5, 239], vec![5], vec![68, 204], vec![127], vec![42, 137], vec![93], vec![90, 258], vec![40, 275], vec![7, 96], vec![108], vec![104, 91], vec![63], vec![31], vec![31, 89], vec![74], vec![81], vec![126, 148], vec![107], vec![13, 28], vec![21, 139], vec![114], vec![5], vec![89], vec![133], vec![20], vec![96, 135], vec![86, 100], vec![83, 75], vec![14], vec![26, 195], vec![37], vec![1, 287], vec![79], vec![15], vec![6], vec![68, 11], vec![52], vec![124, 80], vec![123, 277], vec![99, 281], vec![133], vec![90], vec![45], vec![127], vec![9, 68], vec![123, 6], vec![124, 251], vec![130, 191], vec![23, 174], vec![69, 295], vec![32], vec![37], vec![1, 64], vec![48, 116], vec![68], vec![117, 173], vec![16, 89], vec![84], vec![28, 234], vec![129], vec![89], vec![55], vec![83], vec![99, 264], vec![129], vec![84], vec![14], vec![26, 274], vec![109], vec![110], vec![96, 120], vec![128, 207], vec![12], vec![99, 233], vec![20, 305], vec![26, 24], vec![102, 32], vec![82], vec![16, 30], vec![5, 244], vec![130], vec![109, 36], vec![134, 162], vec![13, 165], vec![45, 235], vec![112, 80], vec![6], vec![34, 98], vec![64, 250], vec![18, 237], vec![72, 21], vec![42, 105], vec![57, 108], vec![28, 229], vec![83], vec![1, 34], vec![93, 151], vec![132, 94], vec![18, 24], vec![57, 68], vec![42, 137], vec![35], vec![80], vec![10, 288], vec![21], vec![115], vec![131], vec![30], vec![43], vec![97, 262], vec![55, 146], vec![81, 112], vec![2, 212], vec![5, 312], vec![82, 107], vec![14, 151], vec![77], vec![60, 42], vec![90, 309], vec![90], vec![131, 220], vec![86], vec![106, 85], vec![85, 254], vec![14], vec![66, 262], vec![88, 243], vec![3], vec![50, 301], vec![118, 91], vec![25], vec![105], vec![100], vec![89], vec![111, 152], vec![65, 24], vec![41, 264], vec![117], vec![117], vec![80, 45], vec![38], vec![11, 151], vec![126, 203], vec![128, 59], vec![6, 129], vec![91], vec![118, 2], vec![50, 164], vec![74], vec![80], vec![48, 308], vec![109, 82], vec![3, 48], vec![123, 10], vec![59, 249], vec![128, 64], vec![41, 287], vec![52, 278], vec![98, 151], vec![12], vec![25], vec![18, 254], vec![24, 40], vec![119], vec![66, 44], vec![61, 19], vec![80, 132], vec![62, 111], vec![80], vec![57, 188], vec![132], vec![42], vec![18, 314], vec![48], vec![86, 138], vec![8], vec![27, 88], vec![96, 178], vec![17, 104], vec![112, 86], vec![25], vec![129, 119], vec![93, 44], vec![115], vec![33, 36], vec![85, 190], vec![10], vec![52, 182], vec![76, 182], vec![109], vec![118], vec![82, 301], vec![26, 158], vec![71], vec![108, 309], vec![58, 132], vec![13, 299], vec![117, 183], vec![115], vec![89], vec![42], vec![11, 285], vec![30, 144], vec![69], vec![31, 53], vec![21], vec![96, 162], vec![4, 227], vec![77, 120], vec![128, 136], vec![92], vec![119, 208], vec![87, 61], vec![9, 40], vec![48, 273], vec![95], vec![35], vec![62, 267], vec![88, 161], vec![59], vec![85], vec![131, 53], vec![114, 98], vec![90, 257], vec![108, 46], vec![54], vec![128, 223], vec![114, 168], vec![89, 203], vec![100], vec![116], vec![14], vec![61, 104], vec![44, 161], vec![60, 132], vec![21, 310], vec![89], vec![109, 237], vec![105], vec![32], vec![78, 101], vec![14, 71], vec![100, 47], vec![102, 33], vec![44, 29], vec![85], vec![37], vec![68, 175], vec![116, 182], vec![42, 47], vec![9], vec![64, 37], vec![23, 32], vec![11, 124], vec![130, 189], vec![65], vec![33, 219], vec![79, 253], vec![80], vec![16], vec![38, 18], vec![35, 67], vec![107], vec![88], vec![37, 13], vec![71, 188], vec![35], vec![58, 268], vec![18, 260], vec![73, 23], vec![28, 102], vec![129], vec![88], vec![65], vec![80], vec![119, 146], vec![113], vec![62], vec![123, 138], vec![18, 1], vec![26, 208], vec![107], vec![107], vec![76, 132], vec![121, 191], vec![4], vec![8], vec![117], vec![11, 118], vec![43], vec![69], vec![136], vec![66, 298], vec![25], vec![71], vec![100], vec![26, 141], vec![53, 256], vec![111, 205], vec![126, 106], vec![43], vec![14, 39], vec![44, 41], vec![23, 230], vec![131], vec![53], vec![104, 268], vec![30], vec![108, 48], vec![72, 45], vec![58], vec![46], vec![128, 301], vec![71], vec![99], vec![113], vec![121], vec![130, 122], vec![102, 5], vec![111, 51], vec![85, 229], vec![86, 157], vec![82, 283], vec![88, 52], vec![136, 105], vec![40], vec![63], vec![114, 244], vec![29, 82], vec![83, 278], vec![131], vec![56, 33], vec![123], vec![11], vec![119], vec![119, 1], vec![48, 52], vec![47], vec![127, 136], vec![78, 38], vec![117, 64], vec![130, 134], vec![93, 69], vec![70, 98], vec![68], vec![4, 3], vec![92, 173], vec![114, 65], vec![7, 309], vec![31], vec![107, 271], vec![110, 69], vec![45], vec![35, 288], vec![20], vec![38, 79], vec![46], vec![6, 123], vec![19], vec![84, 95], vec![76], vec![71, 31], vec![72, 171], vec![35, 123], vec![32], vec![73, 85], vec![94], vec![128], vec![28], vec![38], vec![109], vec![85, 197], vec![10, 41], vec![71, 50], vec![128], vec![3, 55], vec![15, 9], vec![127, 215], vec![17], vec![37], vec![111, 272], vec![79, 169], vec![86, 206], vec![40, 264], vec![134], vec![16, 207], vec![27, 127], vec![29, 48], vec![32, 122], vec![15, 35], vec![117, 36], vec![127], vec![36], vec![72, 70], vec![49, 201], vec![89, 215], vec![134, 290], vec![77, 64], vec![26, 101], vec![99], vec![36, 96], vec![84, 129], vec![125, 264], vec![43], vec![38], vec![24, 76], vec![45, 2], vec![32, 24], vec![84, 235], vec![16, 240], vec![17, 289], vec![49, 94], vec![90, 54], vec![88, 199], vec![23], vec![87, 19], vec![11, 19], vec![24], vec![57], vec![4], vec![40], vec![133, 286], vec![127, 231], vec![51], vec![52, 196], vec![27], vec![10], vec![93], vec![115, 143], vec![62, 64], vec![59, 200], vec![75, 85], vec![7, 93], vec![117, 270], vec![116, 6], vec![32], vec![135], vec![2, 140], vec![23, 1], vec![11, 69], vec![89, 30], vec![27, 14], vec![100], vec![61], vec![99, 41], vec![88, 12], vec![41], vec![52, 203], vec![65], vec![62, 78], vec![104, 276], vec![105, 307], vec![7], vec![23, 123], vec![22], vec![35, 299], vec![69], vec![11], vec![14, 112], vec![115], vec![112], vec![108], vec![110, 165], vec![83, 165], vec![36, 260], vec![54, 73], vec![36], vec![93, 69], vec![134], vec![125, 96], vec![74, 127], vec![110, 305], vec![92, 309], vec![87, 45], vec![31, 266], vec![10], vec![114, 206], vec![49, 141], vec![82], vec![92, 3], vec![91, 160], vec![41], vec![60, 147], vec![36, 239], vec![23, 296], vec![134, 120], vec![6], vec![5, 283], vec![117, 68], vec![35], vec![120], vec![44, 191], vec![121, 14], vec![118, 113], vec![84, 106], vec![23], vec![15, 240], vec![37], vec![52, 256], vec![119, 116], vec![101, 7], vec![14, 157], vec![29, 225], vec![4, 247], vec![8, 112], vec![8, 189], vec![96, 220], vec![104], vec![72, 106], vec![23, 170], vec![67, 209], vec![70, 39], vec![18], vec![6], vec![34], vec![121, 157], vec![16], vec![19], vec![83, 283], vec![13, 22], vec![33, 143], vec![88, 133], vec![88], vec![5, 49], vec![38], vec![110], vec![67], vec![23, 227], vec![68], vec![3], vec![27, 265], vec![31], vec![13, 103], vec![116], vec![111, 282], vec![43, 71], vec![134], vec![70, 141], vec![14], vec![119], vec![43], vec![122], vec![38, 187], vec![8, 9], vec![63], vec![42, 140], vec![83], vec![92], vec![106], vec![28], vec![57, 139], vec![36, 257], vec![30, 204], vec![72], vec![105, 243], vec![16], vec![74, 25], vec![22], vec![118, 144], vec![133], vec![71], vec![99, 21], vec![26], vec![35], vec![89, 209], vec![106, 158], vec![76, 63], vec![112, 216], vec![128], vec![54], vec![16, 165], vec![76, 206], vec![69, 253], vec![23], vec![54, 111], vec![80], vec![111, 72], vec![95, 217], vec![118], vec![4, 146], vec![47], vec![108, 290], vec![43], vec![70, 8], vec![117], vec![121], vec![42, 220], vec![48], vec![32], vec![68, 213], vec![30, 157], vec![62, 68], vec![58], vec![125, 283], vec![132, 45], vec![85], vec![92], vec![23, 257], vec![74], vec![18, 256], vec![90], vec![10, 158], vec![57, 34], vec![27], vec![107]];
        self::test_series_operations(operations, operation_nums);
    }
}



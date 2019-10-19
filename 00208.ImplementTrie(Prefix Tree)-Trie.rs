#[derive(Clone)]
struct TrieNode {
    is_word: bool,
    child: Vec<Option<Box<TrieNode>>>,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            is_word: false,
            child: vec![None; 26],
        }
    }
}

struct Trie {
    root: Box<TrieNode>
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new())
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for i in word.bytes().map(|c| (c as usize - 'a' as usize)) {
            cur = cur.child[i].get_or_insert_with(|| { Box::new(TrieNode::new()) });
        }
        cur.is_word = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.find(word).map_or_else(|| false, |box_trie_node| box_trie_node.is_word)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, prefix: String) -> Option<&Box<TrieNode>> {
        let mut cur = &self.root;
        for i in prefix.bytes().map(|c| (c as usize - 'a' as usize)) {
            match cur.child[i].as_ref() {
                Some(node) => {
                    cur = node
                }
                None => {
                    return None;
                }
            }
        }
        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        trie.search("apple".to_owned());
        trie.starts_with("app".to_owned());
    }
}
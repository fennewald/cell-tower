

pub struct Node {
    pub is_word: bool,
    pub n_children: u32,
    parent: &'static Node,
    children: [Option<&'static Node>; 26],
}

impl Node {
    /// Load the node item with the given prefix
    fn get(&self, prefix: &[u8]) -> Option<&'static Node> {
        let index = (prefix[0] - b'a') as usize;
        if let Some(child) = self.children[index] {
            child.get(&prefix[1..])
        } else {
            None
        }
    }

    /// Determine if the given character is valid at the current node
    pub fn valid_next(&self, c: u8) -> bool {
        let index = (c - b'a') as usize;
        self.children[index].is_some()
    }

    pub fn is_word(&self, word: &[u8]) -> bool {
        if let Some(n) = self.get(word) {
            n.is_word
        } else {
            false
        }
    }
}

pub fn is_word(word: &[u8]) -> bool {
    gen_root.is_word(word)
}

include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));


pub struct Node {
    pub is_word: bool,
    pub n_children: u32,
    children: [*const Node; 26],
}

// Nodes will only ever be made at compile time as static refs
unsafe impl Send for Node {}
unsafe impl Sync for Node {}

impl Node {
    /// Load the node item with the given prefix
    fn get(&'static self, prefix: &[u8]) -> Option<&'static Node> {
        if prefix.len() == 0 {
            return Some(&self);
        }
        let index = (prefix[0] - b'a') as usize;
        if let Some(child) = unsafe{ self.children[index].as_ref() } {
            child.get(&prefix[1..])
        } else {
            None
        }
    }

    /// Determine if the given character is valid at the current node
    pub fn valid_next(&self, c: u8) -> bool {
        let index = (c - b'a') as usize;
        unsafe { self.children[index].as_ref().is_some() }
    }

    /// Return the next node
    pub fn get_next(&'static self, c: u8) -> Option<&'static Node> {
        let index = (c - b'a') as usize;
        unsafe { self.children[index].as_ref() }
    }

    pub fn is_word(&'static self, word: &[u8]) -> bool {
        if let Some(n) = self.get(word) {
            n.is_word
        } else {
            false
        }
    }
}

pub fn first_node(c: u8) -> Option<&'static Node> {
    let buffer = [c];
    GEN__ROOT.get(&buffer)
}

pub fn get_node(word: &[u8]) -> Option<&'static Node> {
    GEN__ROOT.get(word)
}

pub fn is_word(word: &[u8]) -> bool {
    GEN__ROOT.is_word(word)
}

include!(concat!(env!("OUT_DIR"), "/wordlist.rs"));

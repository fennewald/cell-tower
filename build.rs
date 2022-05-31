use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self,BufRead,BufReader,Write,BufWriter};
use std::fmt;

struct Node {
    is_word: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn new() -> Node {
        Node {
            is_word: false,
            children: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None
            ],
        }
    }

    fn add(&mut self, word: &[u8]) {
        if word.len() == 0 {
            self.is_word = true;
        } else {
            let index = (word[0] - b'a') as usize;
            if self.children[index].is_none() {
                self.children[index] = Some(Box::new(Node::new()));
            }
            self.children[index].as_mut().unwrap().add(&word[1..]);
        }
    }

    fn leaf_as_text(&self, prefix: &str, index: usize) -> String {
        if self.children[index].is_some() {
            let c = char::from_u32(index as u32 + b'a' as u32).unwrap();
            format!("Some(gen_{}{})", prefix, c)
        } else {
            "None".to_string()
        }
    }

    fn write_code(&self, f: &mut BufWriter<File>) -> io::Result<()> {
        self.write_code_inner(f, "root", "root", "")
    }

    fn write_code_inner(&self, f: &mut BufWriter<File>, parent: &str, name: &str, base: &str) -> io::Result<()> {
        writeln!(f, "const gen_{}: &'static Node = &Node {{", name)?;
        writeln!(f, "    is_word: {:5?}, n_children: {}, parent: gen_{},", self.is_word, self.n_children(), parent)?;
        writeln!(f, "    children: [")?;
        writeln!(
            f,
            "         {}, {}, {}, {}, {}, {}, {}, {},",
            self.leaf_as_text(base, 0),
            self.leaf_as_text(base, 1),
            self.leaf_as_text(base, 2),
            self.leaf_as_text(base, 3),
            self.leaf_as_text(base, 4),
            self.leaf_as_text(base, 5),
            self.leaf_as_text(base, 6),
            self.leaf_as_text(base, 7),
        )?;
        writeln!(
            f,
            "         {}, {}, {}, {}, {}, {}, {}, {},",
            self.leaf_as_text(base, 8),
            self.leaf_as_text(base, 9),
            self.leaf_as_text(base, 10),
            self.leaf_as_text(base, 11),
            self.leaf_as_text(base, 12),
            self.leaf_as_text(base, 13),
            self.leaf_as_text(base, 14),
            self.leaf_as_text(base, 15),
        )?;
        writeln!(
            f,
            "         {}, {}, {}, {}, {}, {}, {}, {},",
            self.leaf_as_text(base, 16),
            self.leaf_as_text(base, 17),
            self.leaf_as_text(base, 18),
            self.leaf_as_text(base, 19),
            self.leaf_as_text(base, 20),
            self.leaf_as_text(base, 21),
            self.leaf_as_text(base, 22),
            self.leaf_as_text(base, 23),
        )?;
        writeln!(
            f,
            "         {}, {}",
            self.leaf_as_text(base, 24),
            self.leaf_as_text(base, 25)
        )?;
        writeln!(f, "    ],")?;
        writeln!(f, "}};")?;

        for (i, c) in self.children.iter().enumerate() {
            let suffix = char::from_u32(i as u32 + b'a' as u32).unwrap();
            let mut prefix = base.to_string();
            prefix.push(suffix);
            if let Some(child) = c {
                child.write_code_inner(f, name, &prefix, &prefix)?;
            }
        }

        Ok(())
    }

    fn n_children(&self) -> u32 {
        let n = self.children
            .iter()
            .flatten()
            .map(|c| c.n_children())
            .sum();

        if self.is_word {
            n + 1
        } else {
            n
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wordlist.rs");
    let mut root = Node::new();

    // Read word file
    let wordlist = File::open("words.txt").expect("Couldn't open wordlist");
    BufReader::new(wordlist)
        .lines()
        .flatten()
        .for_each(|word| root.add(word.as_bytes()));

    // Write output file
    let out_file = File::create(dest_path).expect("Couldn't create wordlist.rs");
    let mut writer = BufWriter::new(out_file);
    //writeln!(writer, "use crate::dictionary::Node;").expect("Couldn't write header");
    root.write_code(&mut writer).expect("Couldn't write word file");
    println!("cargo:rustc-cfg=has_generated_feature");
}


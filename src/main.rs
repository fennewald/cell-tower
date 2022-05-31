use std::io;

mod dictionary;
// mod board;
// mod cell;
// 
// use board::Board;
// pub use cell::Cell;

fn demo(word: &str) {
    if dictionary::is_word(word.as_bytes()) {
        println!("{} is a word", word);
    } else {
        println!("{} is not a word", word);
    }
}

fn main() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read input line");
        buffer.pop(); // Remove newline
        if let Some(node) = dictionary::get_node(&buffer.as_bytes()) {
            let status = match node.is_word {
                true => "is a word",
                false => "is not a word",
            };
            println!("{} {}, {} children", buffer, status, node.n_children);
        } else {
            println!("{} is not a valid node prefix", buffer);
        }
        buffer.clear();
    }
}

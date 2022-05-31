mod dictionary;
// mod board;
// mod cell;
// 
// use board::Board;
// pub use cell::Cell;

fn demo(word: &str) {
    if dictionary::is_word(word) {
        println!("{} is a word", word);
    } else {
        println!("{} is not a word", word);
    }
}

fn main() {
    demo("hello");
}

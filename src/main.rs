use crate::huffman::encoding;

use std::io;

mod huffman;

fn get_input() -> String {
    let mut input = String::new();

    println!("Insert your message: ");

    io::stdin().read_line(&mut input).expect("Failed to read from stdin!");

    input.trim().to_string()
}

fn main() {
    let message = get_input();
    encoding::encode(&message);
}


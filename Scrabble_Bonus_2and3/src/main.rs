use std::fs::File;
use std::ascii::AsciiExt;
use std::io::{BufRead, BufReader, stdin};
use std::io::prelude::*;
use std::io:: {Error, ErrorKind};

pub mod Bonus2;

fn main() {
    // Read dictionary into a string
    let file = File::open("dic.txt").ok().unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    //Create vector of words
    let mut dictionary_words: Vec<&str> = Bonus2::string_to_vector(&contents);
    // Get Input Chars
    let input = get_input().ok().unwrap();
    // println!("{:?}", dictionary_words);
    let longest_string = Bonus2::longest_word(&input, &dictionary_words);
    println!("The longest word that can be created with the characters, {} is, {}.", input, longest_string);
}

fn get_input() -> Result<String, Error> {
    //Get input chars
    println!("Enter available characters...");
    let mut temp_input_chars = String::new();
    try!(stdin().read_line(&mut temp_input_chars));
    //Return string of chars if is ascii
    if temp_input_chars.is_ascii() {
         Ok(temp_input_chars)
    } else {
        Err(Error::new(ErrorKind::Other, "Not ascii"))
    }
}


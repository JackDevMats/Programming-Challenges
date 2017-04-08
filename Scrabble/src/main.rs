use std::io;

#[derive(Debug)]
struct ScrabbleInput {
    available_chars: String,
    word: String,
}

fn main() {
    //Obtain Input
    let mut input = get_input().ok().unwrap();
    let input_len = [input.available_chars.len(), input.word.len()];
    //Get rid of trailing \r\n
    input.available_chars.truncate(input_len[0] - 2);
    input.word.truncate(input_len[1] - 2);

    //Initial Problem
    if check_chars(&input.available_chars, &input.word) {
        println!("Your word, {}, can be made with the available characters.",
                 &input.available_chars);
    } else {
        println!("Your word, {}, cannot be made with the available characters.",
                 &input.available_chars);
    }
}

fn get_input() -> Result<ScrabbleInput, std::io::Error> {

    println!("Enter available characters...");
    let mut temp_input_chars = String::new();
    try!(io::stdin().read_line(&mut temp_input_chars));

    println!("Enter proposed word to be checked...");
    let mut temp_word_input = String::new();
    try!(io::stdin().read_line(&mut temp_word_input));

    Ok(ScrabbleInput {
           available_chars: temp_input_chars,
           word: temp_word_input,
       })
}

//Check if each character in word is found in chars
fn check_chars(input_chars: &String, word: &String) -> bool {
    for char in word.chars() {
        match input_chars.find(char) {
            None => return false,
            _ => (),
        }
    }
    true
}

fn check_chars_bonus_1(input_chars: &String, word: &String) -> bool {
    let mut find_question_marks = input_chars.clone();
    for char in word.chars() {
        match input_chars.find(char) {
            None => return false,
            _ => ()
        }
    }
}
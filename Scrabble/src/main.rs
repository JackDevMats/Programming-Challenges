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

    //Bonus 1
    if check_chars_bonus_1(&input.available_chars, &input.word) {
        println!("Bonus 1:Your word, {}, can be made with the available characters.",
                 &input.available_chars);
    } else {
        println!("Bonus 1:Your word, {}, cannot be made with the available characters.",
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

//Bonus 1
fn check_chars_bonus_1(input_chars: &String, word: &String) -> bool {
    let mut find_question_marks = input_chars.clone();
    let mut letters_left = word.clone();
    for (index, char) in word.chars().enumerate() {
        match input_chars.find(char) {
            None => (),
            Some(i) => {
                find_question_marks.remove(i);
                let index = letters_left.find(char);
                letters_left.remove(index.unwrap());
                ()
            }
        }
    }
    //Is there enough free letters to make the word
    if number_of_question_marks(input_chars) == letters_left.len() as i32 {
        true
    } else {
        false
    }
}

//Finds number of free letters that can be used to create the input word
fn number_of_question_marks(input: &String) -> i32 {
    let mut count:i32 = 0;
    for char in input.chars() {
        if &*char.to_string() == "?" {
            count +=1;
        }
    }
    count
}
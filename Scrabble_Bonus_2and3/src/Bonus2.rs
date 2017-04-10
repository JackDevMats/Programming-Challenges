pub fn string_to_vector(string: &str) -> Vec<&str> {
   string.lines().collect()
}


pub fn longest_word(input_chars: &String, dictionary: &Vec<&str>) -> String {
	let mut longest_word = String::new();
	for word in dictionary {
		if word.len() > longest_word.len() && input_chars.len() >= word.len() {
			if check_chars(&input_chars.clone(), &word.to_string()) {
				longest_word = word.to_string();
			}
		}
	}
	longest_word
}


//Bonus 1
pub fn check_chars(input_chars: &String, word: &String) -> bool {
    let mut find_question_marks = input_chars.clone();
    let mut letters_left = word.clone();
    for char in word.chars() {
        match find_question_marks.find(char) {
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


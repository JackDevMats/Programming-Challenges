use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

//Store word as string and give it an is_roller property.
struct Rollerword {
    word: String,
    is_roller: bool, 
}

fn main() {
    //Read file into Buffer
    let f = File::open("dic.txt").unwrap();
    let buffer = BufReader::new(f); 
    let mut counter = 0;

    //Itterate over lines in Buffer
    for line in &mut buffer.lines() {
        
        //Store word in Rollerword Structure, and create an array of characters
        let mut word = Rollerword { word: line.unwrap(), is_roller: true};
        let char_array: Vec<_> = word.word.chars().collect();
        //determine if the first letter is before or after the seccond letter in the alphabet
        let mut up = char_array[0] > char_array[1];

        //Itterate over characters, storing both a character and an index
        for (i, c) in char_array.iter().enumerate(){
            //Check for out of bounds
            if i == word.word.len() - 1 {
                break;
            }
            //Check if word follows Rollercoaster word rules
            match up {
                true if (char_array[i+1] < *c) => {
                    up = false;
                }
                false if (char_array [i+1] > *c) => {
                    up = true;
                }
                _ => {
                    word.is_roller = false;                  
                    break;
                }
            }
        }
        //Print word if it is a Rollercoaster word and has a length greater than 4
        if word.is_roller && word.word.len() > 4 {
            println!("{}", word.word);
            counter += 1;
        }
    }
    //How many words passed the test
    println!("{} Rollercoaster Words", counter);
}

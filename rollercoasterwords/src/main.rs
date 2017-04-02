use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Rollerword {
    word: String,
    is_roller: bool, 
}

fn main() {
    
    let f = File::open("dic.txt").unwrap();
    let buffer = BufReader::new(f); 
    let mut counter = 0;
    for line in &mut buffer.lines() {
        
        let mut word = Rollerword { word: line.unwrap(), is_roller: true};
        let char_array: Vec<_> = word.word.clone().chars().collect();
        let mut up = char_array[0] > char_array[1];

        for (i, c) in word.word.clone().chars().enumerate(){
            if i == word.word.len() - 1 {
                break;
            }
            match up {
                true if (char_array[i+1] < c) => {
                    up = false;
                }
                false if (char_array [i+1] > c) => {
                    up = true;
                }
                _ => {
                    word.is_roller = false;                  
                    break;
                }
            }
        }

        if word.is_roller && word.word.len() > 4 {
            println!("{}", word.word);
            counter += 1;
        }
    }
    println!("{}", counter);
}

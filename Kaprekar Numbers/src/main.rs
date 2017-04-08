use std::env::args;
use std::process;

fn main() {
    //Input Parsing
    let mut input = args().skip(1);
    
    //Check for appropriate args length
    if input.size_hint().0 != 2 {
        println!("Enter 2 Arguments, [Lower Bound, Upper Bound] {:?}", input);
        process::exit(1)
    }

    //Get string from input itterator, then parse to i32 and unwrap
    let lower_bound = input.next().unwrap().parse::<i32>().unwrap();
    let higher_bound = input.next().unwrap().parse::<i32>().unwrap();

    //Main loop
    for num in lower_bound..(higher_bound + 1) {
        //Get square of num, then obtain digits of num and store them as chars
        let num_squared = num * num;
        let mut num_squared_digits:Vec<char> = num_squared.to_string().chars().collect();
        

        //Itterate over chars as a splice, parse to i32, then see if they add up to original number
        for (i,_) in num_squared_digits.iter().enumerate() {
            let mut before_split = num_squared_digits.clone();
            let after_split = before_split.split_off(i + 1);
            
            //check if there is nothing in after split -> Due to definition of a Kaprekar number
            //Eg. 10 -> 100 -> 10 + 0 = 10
            //This is not considered a Kaprekar number 
            if after_split.is_empty() || after_split.iter().all(|x| x.to_string() == "0") {
                continue;
            }
            
            //Convert Before and After splits to strings
            let before_num_string:String = before_split.iter().map(|c| *c).collect();
            let after_num_string:String = after_split.iter().map(|c| *c).collect();
            //Convert the string to numbers
            let before_num = before_num_string.parse::<i32>().unwrap();
            let after_num = after_num_string.parse::<i32>().unwrap();
            
            
            //Check if the original number from given range is a Kaprekar Number.
            if before_num + after_num == num {
                println!("{} ", num);
            }
        } 

    }

    
}

use Bonus2::check_chars;

pub struct highscore {
	pub word: String,
	pub score: i32,
}

pub fn highest_score(word: &String, score_reference: &Vec<Vec<char>>, dictionary: &Vec<&str>, input:&String) -> highscore {
	let mut highest_scoring_word = highscore { word: "".to_string(), score: 0i32 };
	for word in dictionary {
		let score = check_score(&word.to_string(), score_reference);
		if score > highest_scoring_word.score {
			if check_chars(input, &word.to_string()) {
				highest_scoring_word.score = score;
				highest_scoring_word.word = word.to_string();
			}
		}
	}
	highest_scoring_word
}

fn check_score(word: &String, score_reference: &Vec<Vec<char>>) -> i32 {
	let mut score = 0;
	for char in word.chars() {
		score += char_to_score(&char, score_reference);
	}
	score
}

fn char_to_score(char_in: &char, score_reference: &Vec<Vec<char>>) -> i32 {
	let mut position = 999;
	for (index, score) in score_reference.iter().enumerate() {
		for char in score {
			if char_in == char {
				position = index;
			}
		}
	};
	match position {
		0 => 1,
		1 => 2,
		2 => 3,
		3 => 4,
		4 => 5,
		5 => 8,
		6 => 10,
		_ => 0,
	}
}


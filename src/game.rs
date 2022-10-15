use colored::Colorize;
use std::io::{self, BufRead, Write};
use crate::loader::load_songs_from_files;
use crate::song::Song;
use crate::guess_generating::{pick_random_guess, optimal_truncated_dist, pick_distractors};
use crate::diff::diff_greedy;
use rand::prelude::SliceRandom;

fn clear_screen() {
	// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	// print!("\x1B[2J\x1B[1;1H");
	// io::stdout().flush().unwrap();
	std::process::Command::new("clear").status().unwrap();

}

pub fn input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().unwrap();
	io::stdin()
		.lock()
		.lines()
		.next()
		.unwrap()
		.map(|x| x.trim_end().to_owned())
		.unwrap()
}

pub fn print_intro() {
	clear_screen();
	println!("{}

In this game, you will be shown a line from a Taylor Swift song. You will then be prompted to enter the following line. If your guess is close enough to the correct answer, you will score a certain number of points (and you earn a bonus if your guess is a perfect match).

If you do not know what the next line is, simply enter a question mark ('?') and you will be given 17 choices for the next line. Upon seeing the choices, enter a number 1-17, and you will score only 1 point for the correct answer.

After each round, press the enter key to continue, or enter '?' to view the full lyrics of the song.

The game ends as soon as you submit an incorrect answer.

Good luck! And have fun!", 

"Welcome to a Taylor Swift lyric guessing game!".blue().bold());

	input("Press enter to begin.");
	clear_screen();
}

fn print_with_flags(text: &str, flags: Vec<i32>) {
	if text.chars().count() != flags.len() {
		panic!("called print_with_flags with mismatched text and flag");
	}
	for i in 0..flags.len() {
		match flags[i] {
			1 => {
				print!("{}", text.chars().nth(i).unwrap().to_string().red().bold())
			},
			2 => {
				print!("{}", text.chars().nth(i).unwrap().to_string().yellow().bold())
			}
			_ => {
				print!("{}", text.chars().nth(i).unwrap())
			},
		}
	}
	println!("");
}

fn print_guess_with_answer(guess: &str, answer: &str, optimal_truncate_amt: i32) {
	let (_, diffs) = 
		diff_greedy(
			&guess.to_lowercase()[0..(guess.len()-optimal_truncate_amt as usize)], 
			&answer.to_lowercase(),
		).unwrap();
	
	let mut guess_flags = vec![0; guess.chars().count()];
	let mut ans_flags = vec![0; answer.chars().count()];
	for insertion in diffs.get("insert").unwrap() {
		for i in insertion.at..=insertion.to {
			ans_flags[i] = 1;
		}
	}
	for deletion in diffs.get("delete").unwrap() {
		for i in deletion.at..=deletion.to {
			guess_flags[i] = 1;
		}
	}
	for i in (guess_flags.len()-optimal_truncate_amt as usize)..guess_flags.len() {
		guess_flags[i] = 2;
	}

	print!("{}", "   Your Answer: ".blue().bold());
	print_with_flags(guess, guess_flags);
	print!("{}", "Correct Answer: ".blue().bold());
	print_with_flags(answer, ans_flags);
}


fn print_song(song: &Song, highlighted_line: &str) {
	println!("{}", format!("{} : {}", song.album, song.name).green().bold());
	for line in song.lyrics_raw.split("\n") {
		if line.trim() == highlighted_line {
			println!("{}", line.green().bold());
		} else {
			println!("{}", line);
		}

	}
}

fn take_user_multiple_choice_guess(answer: &str, choices: &Vec<String>) -> bool{
	// returns true if the user gets the correct answer
	for i in 0..choices.len(){
			println!("{}{} {}",
				(i+1).to_string().blue().bold(),
				")".blue().bold(),
				choices[i]
			);
	}
	let acceptable_inputs: Vec<String> = (1..=choices.len()).map(|x| x.to_string()).collect();
	let chosen_index: usize;
	loop {
		let guess = input(">>> ");
		if guess == "/" {
			return false;
		}
		if !acceptable_inputs.contains(&guess) {
			println!("That's not a valid choice!")
		} else {
			chosen_index = guess.parse::<usize>().unwrap() - 1;
			break
		}

	}

	return choices[chosen_index] == answer;
}


pub fn run_game_loop() {
	
	const MAX_ACCEPTABLE_DIST: usize = 13;
	const POINTS_FOR_PERFECT_MATCH: i32 = 26;
	
	let mut score = 0;
	let songs: Vec<Song> = load_songs_from_files();
	print_intro();

	loop {
		let question = pick_random_guess(&songs);
		clear_screen();
		println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
		
		let dist: usize;

		let mut guess = input(">>> ");
		loop {
			if guess == "?" {
				println!("Reducing to a multiple choice challenge: ");
				let mut choices = pick_distractors(&question.answer, &songs);
				choices.push(question.answer.to_owned());
				choices.shuffle(&mut rand::thread_rng());

				let result = take_user_multiple_choice_guess(&question.answer, &choices);
				dist = if result { MAX_ACCEPTABLE_DIST } else { MAX_ACCEPTABLE_DIST + 1 };
				
				break;
			}

			else if guess.chars().count() < question.answer.chars().count() - 5 && guess != "/" {
				println!("Try guessing again: your guess was significantly shorter than the programmed answer");
				guess = input(">>> ");
			} else {
				let (truncate_amt, dist_after_truncation) = optimal_truncated_dist(&guess, &question.answer);
		
				print_guess_with_answer(&guess, &question.answer, truncate_amt);

				dist = dist_after_truncation;
				break;
			}
		}



		if dist > MAX_ACCEPTABLE_DIST {
			println!("That wasn't it! The game is over now, thanks for playing!");
			let response = input("Press enter to quit ('?' to show song, 'r' to restart): ");
			if response == "?" {
				print_song(&question.song, &question.shown_line);
				if input("Press enter to quit, r to restart:") == "r" {
					score = 0;
					continue;
				}
			} else if response == "r" {
				score = 0;
				continue;
			}
			std::process::exit(0);
		} else if dist != 0 {
			let points_earned = (MAX_ACCEPTABLE_DIST - dist + 1) as i32;
			println!("Correct! You scored {points_earned} points for your answer.");
			score += points_earned;
		} else {
			println!("Yes! You scored {POINTS_FOR_PERFECT_MATCH} points for your {}!", "perfect match".green().bold());
			score += POINTS_FOR_PERFECT_MATCH;
		}


		// println!("The correct answer was {} and the distance was {}, {}", question.answer, dist, truncate_amt);
		let response = input("Press enter to continue ('?' to show song): ");
		if response == "?" {
			print_song(&question.song, &question.shown_line);
			input("Press enter to continue: ");
		}
	}
}

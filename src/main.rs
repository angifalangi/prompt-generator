use std::io;

use rand::prelude::SliceRandom;

use crate::{
	adjectives::ADJECTIVES, nouns::NOUNS, objects::OBJECTS, settings::PHRASES, verbs::VERBS,
};

mod adjectives;
mod nouns;
mod objects;
mod settings;
mod verbs;

/// Checks the first character of the previous word to select the proper article.
fn choose_article(word: &str) -> &str {
	static VOWELS: &str = "aeiou";

	let first = word.chars().next().unwrap();

	if VOWELS.contains(first) {
		"An"
	} else {
		"A"
	}
}

/// Selects a random word from the given slice.
fn select_word<'a>(word: &[&'a str]) -> &'a str {
	let mut rng = rand::thread_rng();

	word.choose(&mut rng).unwrap()
}

/// Generates a prompt.
fn generate() {
	let adjective = select_word(ADJECTIVES);
	let noun = select_word(NOUNS);
	let verb = select_word(VERBS);
	let object = select_word(OBJECTS);
	let setting = select_word(PHRASES);

	let article_1 = choose_article(adjective);
	let article_2 = choose_article(object);

	println!(
		"{} {} {} with {} {} {} {}",
		article_1,
		adjective,
		noun,
		article_2.to_lowercase(),
		object,
		verb,
		setting
	);
}

fn main() {
	println!("Welcome. How many prompts would you like to generate?");
	println!("You can generate up to 10 prompts at a time.");

	let amount: i32;

	loop {
		let mut input = String::new();

		io::stdin().read_line(&mut input).unwrap();

		if let Ok(temp) = input.trim().parse() {
			if temp <= 10 {
				amount = temp;
				break;
			}

			println!("Too many prompts! Try again with a lower value:");
		}
	}

	for _ in 0..amount {
		generate();
	}
}

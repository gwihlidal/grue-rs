use rand::prelude::*;
use regex::{Captures, Regex};
use serde::Deserialize;

macro_rules! random_choice {
	($self_:ident, $a:ident) => {
		if $self_.$a.len() > 0 {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0..$self_.$a.len());
			$self_.$a[choice].to_owned()
		} else {
			String::default()
		}
	};
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Tavern {
	pub patterns: Vec<String>,
	pub nouns: Vec<String>,
	pub adjectives: Vec<String>,
	pub titles: Vec<String>,
}

impl Tavern {
	pub fn new(
		patterns: &[String],
		nouns: &[String],
		adjectives: &[String],
		titles: &[String],
	) -> Tavern {
		Tavern {
			patterns: patterns.to_vec(),
			nouns: nouns.to_vec(),
			adjectives: adjectives.to_vec(),
			titles: titles.to_vec(),
		}
	}

	pub fn random_noun(&self) -> String {
		random_choice!(self, nouns)
	}

	pub fn random_adjective(&self) -> String {
		random_choice!(self, adjectives)
	}

	pub fn random_title(&self) -> String {
		random_choice!(self, titles)
	}

	pub fn random_name(
		&self,
		starts_with: &str,
		ends_with: &str,
		contains: &str,
		does_not_contain: &str,
	) -> String {
		let lower_starts_with: &str = &starts_with.to_lowercase();
		let lower_ends_with: &str = &ends_with.to_lowercase();
		let lower_contains: &str = &contains.to_lowercase();
		let lower_does_not_contain: &str = &does_not_contain.to_lowercase();

		// Rewrite this to be more rust idiomatic...
		let mut i = 0;
		let mut name = String::default();

		let re = Regex::new(r"<([\w\W]*?)>").unwrap();

		// Try to get result for random patterns, stop after 100 attempts.
		while name.is_empty() && i < 100 {
			i += 1;

			let pattern = self.random_pattern();
			//println!("pattern [{}]: {}", i, pattern);

			let result = re.replace_all(&pattern, |caps: &Captures| {
				let token = &caps[0];
				let result = match token {
					"<adjective>" => self.random_adjective(),
					"<noun>" => self.random_noun(),
					"<title>" => self.random_title(),
					_ => "INVALID_PATTERN".to_string(),
				};

				if result.is_empty() {
					"INVALID_PATTERN".to_string()
				} else {
					result.trim().to_string()
				}
			});

			if result != pattern
				&& self.valid_name(
					&result,
					lower_starts_with,
					lower_ends_with,
					lower_contains,
					lower_does_not_contain,
				) {
				name = result.to_string();
			}
		}

		name
	}

	pub fn random_pattern(&self) -> String {
		random_choice!(self, patterns)
	}

	fn valid_name(
		&self,
		name: &str,
		starts_with: &str,
		ends_with: &str,
		contains: &str,
		does_not_contain: &str,
	) -> bool {
		if name.contains("INVALID_PATTERN") {
			false
		} else {
			let lower_name: &str = &name.to_lowercase();
			let check0 = !starts_with.is_empty() && !lower_name.starts_with(starts_with);
			let check1 = !ends_with.is_empty() && !name.to_lowercase().ends_with(ends_with);
			let check2 = !contains.is_empty() && !name.to_lowercase().contains(contains);
			let check3 =
				!does_not_contain.is_empty() && name.to_lowercase().contains(does_not_contain);
			!(check0 || check1 || check2 || check3)
		}
	}
}

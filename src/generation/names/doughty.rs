use rand::prelude::*;
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
pub struct Doughty {
	pub syllables: Vec<String>,
	pub male_suffixes: Vec<String>,
	pub female_suffixes: Vec<String>,
}

impl Doughty {
	pub fn random_syllable(&self) -> String {
		random_choice!(self, syllables)
	}

	pub fn random_male_suffix(&self) -> String {
		random_choice!(self, male_suffixes)
	}

	pub fn random_female_suffix(&self) -> String {
		random_choice!(self, female_suffixes)
	}
}

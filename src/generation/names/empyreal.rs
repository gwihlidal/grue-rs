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
pub struct Empyreal {
	pub prefixes: Vec<String>,
	pub male_suffixes: Vec<String>,
	pub female_suffixes: Vec<String>,
	pub titles_ar: Vec<String>,
	pub titles_al: Vec<String>,
}

impl Empyreal {
	pub fn random_prefix(&self) -> String {
		random_choice!(self, prefixes)
	}

	pub fn random_male_suffix(&self) -> String {
		random_choice!(self, male_suffixes)
	}

	pub fn random_female_suffix(&self) -> String {
		random_choice!(self, female_suffixes)
	}

	pub fn random_title_ar(&self) -> String {
		random_choice!(self, titles_ar)
	}

	pub fn random_title_al(&self) -> String {
		random_choice!(self, titles_al)
	}

	pub fn random_title(&self) -> String {
		if !self.titles_ar.is_empty() || !self.titles_al.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0..self.titles_ar.len() + self.titles_al.len());
			if choice < self.titles_ar.len() {
				self.titles_ar[choice].to_owned()
			} else {
				self.titles_al[choice - self.titles_ar.len()].to_owned()
			}
		} else {
			String::default()
		}
	}
}

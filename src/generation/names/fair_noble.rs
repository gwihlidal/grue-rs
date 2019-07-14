use rand::prelude::*;

macro_rules! random_choice {
	($self_:ident, $a:ident) => {
		if $self_.$a.len() > 0 {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, $self_.$a.len());
			$self_.$a[choice].to_owned()
		} else {
			String::default()
			}
	};
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct FairAndNoble {
	pub elf_prefixes: Vec<String>,
	pub elf_prefixes_alternative: Vec<String>,
	pub middle: Vec<String>,
	pub male_suffixes: Vec<String>,
	pub female_suffixes: Vec<String>,
}

impl FairAndNoble {
	pub fn random_elf_prefix(&self) -> String {
		random_choice!(self, elf_prefixes)
	}

	pub fn random_elf_prefix_alternative(&self) -> String {
		random_choice!(self, elf_prefixes_alternative)
	}

	pub fn random_middle(&self) -> String {
		random_choice!(self, middle)
	}

	pub fn random_male_suffix(&self) -> String {
		random_choice!(self, male_suffixes)
	}

	pub fn random_female_suffix(&self) -> String {
		random_choice!(self, female_suffixes)
	}
}

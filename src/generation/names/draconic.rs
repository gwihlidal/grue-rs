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
pub struct Draconic {
	pub prefixes: Vec<String>,
	pub suffixes: Vec<String>,
}

impl Draconic {
	pub fn random_prefix(&self) -> String {
		random_choice!(self, prefixes)
	}

	pub fn random_suffix(&self) -> String {
		random_choice!(self, suffixes)
	}
}

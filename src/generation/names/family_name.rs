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
pub struct FamilyName {
	pub english: Vec<String>,
	pub scottish: Vec<String>,
}

impl FamilyName {
	pub fn random_english(&self) -> String {
		random_choice!(self, english)
	}

	pub fn random_scottish(&self) -> String {
		random_choice!(self, scottish)
	}
}

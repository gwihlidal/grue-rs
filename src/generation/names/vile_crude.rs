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
pub struct VileAndCrude {
	pub small: Vec<String>,
	pub medium: Vec<String>,
	pub large: Vec<String>,
}

impl VileAndCrude {
	pub fn random_small(&self) -> String {
		random_choice!(self, small)
	}

	pub fn random_medium(&self) -> String {
		random_choice!(self, medium)
	}

	pub fn random_large(&self) -> String {
		random_choice!(self, large)
	}
}

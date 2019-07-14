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
pub struct Infernal {
	pub softs: Vec<String>,
	pub dulls: Vec<String>,
	pub sharps: Vec<String>,
}

impl Infernal {
	pub fn random_soft(&self) -> String {
		random_choice!(self, softs)
	}

	pub fn random_dull(&self) -> String {
		random_choice!(self, dulls)
	}

	pub fn random_sharp(&self) -> String {
		random_choice!(self, sharps)
	}
}

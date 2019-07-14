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
pub struct Primitive {
	pub names: Vec<String>,
	pub suffixes: Vec<String>,
}

impl Primitive {
	pub fn random_name(&self) -> String {
		random_choice!(self, names)
	}

	pub fn random_suffix(&self) -> String {
		random_choice!(self, suffixes)
	}
}

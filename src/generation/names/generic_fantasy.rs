use rand::prelude::*;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct GenericFantasy {
	pub one_syllable: Vec<String>,
	pub two_syllables: Vec<String>,
	pub three_syllables: Vec<String>,
	pub many_syllables: Vec<String>,
}

impl GenericFantasy {
	pub fn new(
		one_syllable: &[String],
		two_syllables: &[String],
		three_syllables: &[String],
		many_syllables: &[String],
	) -> GenericFantasy {
		GenericFantasy {
			one_syllable: one_syllable.to_vec(),
			two_syllables: two_syllables.to_vec(),
			three_syllables: three_syllables.to_vec(),
			many_syllables: many_syllables.to_vec(),
		}
	}

	pub fn random_name(&self) -> String {
		let mut rng = thread_rng();
		let d20 = rng.gen_range(0, 21);
		match d20 {
			0..=2 => {
				// 10%
				self.random_one_syllable()
			}
			3..=11 => {
				// 45%
				self.random_two_syllables()
			}
			12..=16 => {
				// 25%
				self.random_three_syllables()
			}
			17 => {
				// 5%
				self.random_many_syllables()
			}
			18 => {
				// 5%
				format!(
					"{} {}",
					self.random_one_syllable(),
					self.random_two_syllables()
				)
			}
			19 => {
				// 5%
				format!(
					"{} {}",
					self.random_two_syllables(),
					self.random_one_syllable()
				)
			}
			_ => {
				// 5%
				let first_name = self.random_any_syllables();
				let second_name = self.random_any_syllables();
				format!("{} {}", first_name, second_name)
			}
		}
	}

	pub fn random_one_syllable(&self) -> String {
		let mut rng = thread_rng();
		self.one_syllable[rng.gen_range(0, self.one_syllable.len())].to_owned()
	}

	pub fn random_two_syllables(&self) -> String {
		let mut rng = thread_rng();
		self.two_syllables[rng.gen_range(0, self.two_syllables.len())].to_owned()
	}

	pub fn random_three_syllables(&self) -> String {
		let mut rng = thread_rng();
		self.three_syllables[rng.gen_range(0, self.three_syllables.len())].to_owned()
	}

	pub fn random_many_syllables(&self) -> String {
		let mut rng = thread_rng();
		self.many_syllables[rng.gen_range(0, self.many_syllables.len())].to_owned()
	}

	pub fn random_any_syllables(&self) -> String {
		let mut rng = thread_rng();
		match rng.gen_range(0, 5) {
			0 => self.random_one_syllable(),
			1 => self.random_two_syllables(),
			2 => self.random_three_syllables(),
			_ => self.random_many_syllables(),
		}
	}
}

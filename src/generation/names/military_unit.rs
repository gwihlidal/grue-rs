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
pub struct MilitaryUnit {
	pub patterns: Vec<String>,
	pub teams: Vec<String>,
	pub soldiers: Vec<String>,
	pub warders: Vec<String>,
	pub mercenaries: Vec<String>,
	pub gear: Vec<String>,
	pub creatures: Vec<String>,
	pub colours: Vec<String>,
	pub other: Vec<String>,
	pub places: Vec<String>,
	pub lands: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
impl MilitaryUnit {
	pub fn new(
		patterns: &[String],
		teams: &[String],
		soldiers: &[String],
		warders: &[String],
		mercenaries: &[String],
		gear: &[String],
		creatures: &[String],
		colours: &[String],
		other: &[String],
		places: &[String],
		lands: &[String],
	) -> MilitaryUnit {
		MilitaryUnit {
			patterns: patterns.to_vec(),
			teams: teams.to_vec(),
			soldiers: soldiers.to_vec(),
			warders: warders.to_vec(),
			mercenaries: mercenaries.to_vec(),
			gear: gear.to_vec(),
			creatures: creatures.to_vec(),
			colours: colours.to_vec(),
			other: other.to_vec(),
			places: places.to_vec(),
			lands: lands.to_vec(),
		}
	}

	pub fn random_pattern(&self) -> String {
		let mut rng = thread_rng();
		if self.patterns.len() >= 4 {
			let d10 = rng.gen_range(0..10);
			match d10 {
				0..=1 => self.patterns[0].to_owned(),
				2..=7 => self.patterns[1].to_owned(),
				8..=9 => self.patterns[2].to_owned(),
				_ => self.patterns[3].to_owned(),
			}
		} else if !self.patterns.is_empty() {
			let choice = rng.gen_range(0..self.patterns.len());
			self.patterns[choice].to_owned()
		} else {
			String::default()
		}
	}

	pub fn random_group(&self) -> String {
		let mut rng = thread_rng();
		let d6 = rng.gen_range(0..6);
		match d6 {
			0 => self.random_team(),
			1 => self.random_soldier(),
			2 => self.random_warder(),
			3 => self.random_mercenary(),
			4 => self.random_gear(),
			_ => self.random_creature(),
		}
	}

	pub fn random_descriptive(&self) -> String {
		let mut rng = thread_rng();
		if rng.gen() {
			self.random_colour()
		} else {
			self.random_other()
		}
	}

	pub fn random_location(&self) -> String {
		let mut rng = thread_rng();
		if rng.gen() {
			self.random_place()
		} else {
			self.random_land()
		}
	}

	pub fn random_team(&self) -> String {
		random_choice!(self, teams)
	}

	pub fn random_soldier(&self) -> String {
		random_choice!(self, soldiers)
	}

	pub fn random_warder(&self) -> String {
		random_choice!(self, warders)
	}

	pub fn random_mercenary(&self) -> String {
		random_choice!(self, mercenaries)
	}

	pub fn random_gear(&self) -> String {
		random_choice!(self, gear)
	}

	pub fn random_creature(&self) -> String {
		random_choice!(self, creatures)
	}

	pub fn random_colour(&self) -> String {
		random_choice!(self, colours)
	}

	pub fn random_other(&self) -> String {
		random_choice!(self, other)
	}

	pub fn random_place(&self) -> String {
		random_choice!(self, places)
	}

	pub fn random_land(&self) -> String {
		random_choice!(self, lands)
	}
}

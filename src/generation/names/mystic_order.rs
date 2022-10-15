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
pub struct MysticOrder {
	pub patterns: Vec<String>,
	pub cliques: Vec<String>,
	pub people: Vec<String>,
	pub qualities: Vec<String>,
	pub colours: Vec<String>,
	pub entities: Vec<String>,
}

impl MysticOrder {
	pub fn new(
		patterns: &[String],
		cliques: &[String],
		people: &[String],
		qualities: &[String],
		colours: &[String],
		entities: &[String],
	) -> MysticOrder {
		MysticOrder {
			patterns: patterns.to_vec(),
			cliques: cliques.to_vec(),
			people: people.to_vec(),
			qualities: qualities.to_vec(),
			colours: colours.to_vec(),
			entities: entities.to_vec(),
		}
	}

	pub fn random_pattern(&self) -> String {
		random_choice!(self, patterns)
	}

	pub fn random_descriptive(&self) -> String {
		// 50/50 for quality vs. colour
		let mut rng = thread_rng();
		if rng.gen() {
			self.random_quality()
		} else {
			self.random_colour()
		}
	}

	pub fn random_group(&self) -> String {
		// 50/50 for cliques vs. people
		let mut rng = thread_rng();
		if rng.gen() {
			self.random_clique()
		} else {
			self.random_person()
		}
	}

	pub fn random_clique(&self) -> String {
		random_choice!(self, cliques)
	}

	pub fn random_person(&self) -> String {
		random_choice!(self, people)
	}

	pub fn random_quality(&self) -> String {
		random_choice!(self, qualities)
	}

	pub fn random_colour(&self) -> String {
		random_choice!(self, colours)
	}

	pub fn random_entity(&self) -> String {
		random_choice!(self, entities)
	}
}

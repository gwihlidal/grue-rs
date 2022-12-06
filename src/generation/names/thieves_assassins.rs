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

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThievesAssassins {
	pub roles: Vec<String>,
	pub goals: Vec<String>,
	pub adjectives: Vec<String>,
	pub directives: Vec<String>,
	pub titles: Vec<String>,
	pub descriptives: Vec<String>,
	pub weapons: Vec<String>,
	pub items: Vec<String>,
	pub creatures: Vec<String>,
	pub actions: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
impl ThievesAssassins {
	pub fn new(
		roles: &[String],
		goals: &[String],
		adjectives: &[String],
		directives: &[String],
		titles: &[String],
		descriptives: &[String],
		weapons: &[String],
		items: &[String],
		creatures: &[String],
		actions: &[String],
	) -> ThievesAssassins {
		ThievesAssassins {
			roles: roles.to_vec(),
			goals: goals.to_vec(),
			adjectives: adjectives.to_vec(),
			directives: directives.to_vec(),
			titles: titles.to_vec(),
			descriptives: descriptives.to_vec(),
			weapons: weapons.to_vec(),
			items: items.to_vec(),
			creatures: creatures.to_vec(),
			actions: actions.to_vec(),
		}
	}

	pub fn random_group(&self) -> String {
		let mut rng = thread_rng();
		let d4 = rng.gen_range(0..4);
		match d4 {
			0 => self.random_weapon(),
			1 => self.random_item(),
			2 => self.random_creature(),
			_ => self.random_action(),
		}
	}

	pub fn random_role(&self) -> String {
		random_choice!(self, roles)
	}

	pub fn random_goal(&self) -> String {
		random_choice!(self, goals)
	}

	pub fn random_adjective(&self) -> String {
		random_choice!(self, adjectives)
	}

	pub fn random_directive(&self) -> String {
		random_choice!(self, directives)
	}

	pub fn random_title(&self) -> String {
		random_choice!(self, titles)
	}

	pub fn random_descriptive(&self) -> String {
		random_choice!(self, descriptives)
	}

	pub fn random_weapon(&self) -> String {
		random_choice!(self, weapons)
	}

	pub fn random_item(&self) -> String {
		random_choice!(self, items)
	}

	pub fn random_creature(&self) -> String {
		random_choice!(self, creatures)
	}

	pub fn random_action(&self) -> String {
		random_choice!(self, actions)
	}

	pub fn random_name(&self) -> String {
		let mut rng = thread_rng();
		let d30 = rng.gen_range(0..30);
		match d30 {
			0..=5 => format!("{} of {}", self.random_role(), self.random_goal()),
			6..=10 => format!(
				"{} {} {}",
				self.random_adjective(),
				self.random_directive(),
				self.random_title()
			),
			_ => format!("{} {}", self.random_descriptive(), self.random_group()),
		}
	}
}

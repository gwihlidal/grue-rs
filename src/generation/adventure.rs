use rand::prelude::*;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Entry {
	pub name: String,
	pub desc: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Adventures {
	pub themes: Vec<Entry>,
	pub goals: Vec<Entry>,
	pub story_hooks: Vec<Entry>,
	pub plots: Vec<Entry>,
	pub climaxes: Vec<Entry>,
	pub general_settings: Vec<Entry>,
	pub specific_settings: Vec<Entry>,
	pub master_villains: Vec<Entry>,
	pub minor_villains: Vec<Entry>,
	pub allies_neutral: Vec<Entry>,
	pub monster_encounters: Vec<Entry>,
	pub character_encounters: Vec<Entry>,
	pub death_traps: Vec<Entry>,
	pub chases: Vec<Entry>,
	pub omens_prophecies: Vec<Entry>,
	pub secret_weaknesses: Vec<Entry>,
	pub special_conditions: Vec<Entry>,
	pub moral_quandaries: Vec<Entry>,
	pub red_herrings: Vec<Entry>,
	pub cruel_tricks: Vec<Entry>,
}

impl Adventures {
	pub fn random_theme(&self) -> (String, String) {
		if !self.themes.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.themes.len());
			(
				self.themes[choice].name.to_owned(),
				self.themes[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_goal(&self) -> (String, String) {
		if !self.goals.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.goals.len());
			(
				self.goals[choice].name.to_owned(),
				self.goals[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_story_hook(&self) -> (String, String) {
		if !self.story_hooks.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.story_hooks.len());
			(
				self.story_hooks[choice].name.to_owned(),
				self.story_hooks[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_plot(&self) -> (String, String) {
		if !self.plots.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.plots.len());
			(
				self.plots[choice].name.to_owned(),
				self.plots[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_climax(&self) -> (String, String) {
		if !self.climaxes.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.climaxes.len());
			(
				self.climaxes[choice].name.to_owned(),
				self.climaxes[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_general_setting(&self) -> (String, String) {
		if !self.general_settings.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.general_settings.len());
			(
				self.general_settings[choice].name.to_owned(),
				self.general_settings[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_specific_setting(&self) -> (String, String) {
		if !self.specific_settings.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.specific_settings.len());
			(
				self.specific_settings[choice].name.to_owned(),
				self.specific_settings[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_master_villain(&self) -> (String, String) {
		if !self.master_villains.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.master_villains.len());
			(
				self.master_villains[choice].name.to_owned(),
				self.master_villains[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_minor_villain(&self) -> (String, String) {
		if !self.minor_villains.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.minor_villains.len());
			(
				self.minor_villains[choice].name.to_owned(),
				self.minor_villains[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_ally_neutral(&self) -> (String, String) {
		if !self.allies_neutral.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.allies_neutral.len());
			(
				self.allies_neutral[choice].name.to_owned(),
				self.allies_neutral[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_monster_encounter(&self) -> (String, String) {
		if !self.monster_encounters.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.monster_encounters.len());
			(
				self.monster_encounters[choice].name.to_owned(),
				self.monster_encounters[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_character_encounter(&self) -> (String, String) {
		if !self.character_encounters.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.character_encounters.len());
			(
				self.character_encounters[choice].name.to_owned(),
				self.character_encounters[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_death_trap(&self) -> (String, String) {
		if !self.death_traps.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.death_traps.len());
			(
				self.death_traps[choice].name.to_owned(),
				self.death_traps[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_chase(&self) -> (String, String) {
		if !self.chases.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.chases.len());
			(
				self.chases[choice].name.to_owned(),
				self.chases[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_omen_prophecy(&self) -> (String, String) {
		if !self.omens_prophecies.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.omens_prophecies.len());
			(
				self.omens_prophecies[choice].name.to_owned(),
				self.omens_prophecies[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_secret_weakness(&self) -> (String, String) {
		if !self.secret_weaknesses.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.secret_weaknesses.len());
			(
				self.secret_weaknesses[choice].name.to_owned(),
				self.secret_weaknesses[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_special_condition(&self) -> (String, String) {
		if !self.special_conditions.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.omens_prophecies.len());
			(
				self.special_conditions[choice].name.to_owned(),
				self.special_conditions[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_moral_quandary(&self) -> (String, String) {
		if !self.moral_quandaries.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.moral_quandaries.len());
			(
				self.moral_quandaries[choice].name.to_owned(),
				self.moral_quandaries[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_red_herring(&self) -> (String, String) {
		if !self.red_herrings.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.red_herrings.len());
			(
				self.red_herrings[choice].name.to_owned(),
				self.red_herrings[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}

	pub fn random_cruel_trick(&self) -> (String, String) {
		if !self.cruel_tricks.is_empty() {
			let mut rng = thread_rng();
			let choice = rng.gen_range(0, self.cruel_tricks.len());
			(
				self.cruel_tricks[choice].name.to_owned(),
				self.cruel_tricks[choice].desc.to_owned(),
			)
		} else {
			(String::default(), String::default())
		}
	}
}

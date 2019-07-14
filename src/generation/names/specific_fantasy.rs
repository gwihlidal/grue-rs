use super::*;
use rand::prelude::*;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SpecificFantasy {
	pub vile_and_crude: VileAndCrude,
	pub primitive: Primitive,
	pub doughty: Doughty,
	pub homely: Homely,
	pub family_name: FamilyName,
	pub fair_and_noble: FairAndNoble,
	pub faery_kind: FaeryKind,
	pub alternative_faery_kind: AlternativeFaeryKind,
	pub elegant_evil: ElegantEvil,
	pub malevolent: Malevolent,
	pub draconic: Draconic,
	pub infernal: Infernal,
	pub empyreal: Empyreal,
}

impl SpecificFantasy {
	pub fn goblin_name(&self) -> String {
		format!(
			"{}{}",
			self.vile_and_crude.random_small(),
			self.vile_and_crude.random_small()
		)
	}

	pub fn orc_name(&self) -> String {
		format!(
			"{}{}",
			self.vile_and_crude.random_medium(),
			self.vile_and_crude.random_medium()
		)
	}

	pub fn ogre_name(&self) -> String {
		format!(
			"{}{}",
			self.vile_and_crude.random_large(),
			self.vile_and_crude.random_large()
		)
	}

	pub fn caveman_male_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = self.primitive.random_name();
		if d10 > 3 {
			name += &format!("-{}", self.primitive.random_name());
		}
		if d10 > 8 {
			name += &format!("-{}", self.primitive.random_name());
		}
		name
	}

	pub fn caveman_female_name(&self) -> String {
		let mut rng = thread_rng();
		let mut name = self.primitive.random_name();
		if rng.gen() {
			name += &format!("-{}", self.primitive.random_name());
		}
		name += &format!("-{}", self.primitive.random_suffix());
		name
	}

	pub fn dwarf_male_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = self.doughty.random_syllable();
		if d10 > 8 {
			if is_vowel(name.chars().last().unwrap()) {
				name.push('r');
			} else {
				name.push('i');
			}
		} else {
			name += &format!("-{}", self.doughty.random_male_suffix());
		}

		name
	}

	pub fn dwarf_female_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = self.doughty.random_syllable();
		if d10 > 8 {
			if is_vowel(name.chars().last().unwrap()) {
				name.push('r');
				name.push('a');
			} else {
				name.push('a');
			}
		} else {
			name += &format!("-{}", self.doughty.random_female_suffix());
		}

		name
	}

	pub fn halfling_male_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = format!(
			"{}{}",
			self.homely.random_syllable(),
			self.homely.random_male_suffix()
		);
		if d10 > 7 {
			name += &format!(" {}", self.family_name.random_english());
		}
		name
	}

	pub fn halfling_female_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = format!(
			"{}{}",
			self.homely.random_syllable(),
			self.homely.random_female_suffix()
		);
		if d10 > 7 {
			name += &format!(" {}", self.family_name.random_english());
		}
		name
	}

	pub fn gnome_male_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = self.doughty.random_syllable();
		if is_vowel(name.chars().last().unwrap()) {
			name.push('l');
		}
		name += &format!("-{}", self.homely.random_male_suffix());
		if d10 > 7 {
			name += &format!("-{}", self.family_name.random_scottish());
		}
		name
	}

	pub fn gnome_female_name(&self) -> String {
		let mut rng = thread_rng();
		let d10 = rng.gen_range(0, 10);
		let mut name = self.doughty.random_syllable();
		if is_vowel(name.chars().last().unwrap()) {
			name.push('l');
		}
		name += &format!("-{}", self.homely.random_female_suffix());
		if d10 > 7 {
			name += &format!("-{}", self.family_name.random_scottish());
		}
		name
	}

	pub fn elf_male_name(&self) -> String {
		format!(
			"{}{}{}",
			self.fair_and_noble.random_elf_prefix(),
			self.fair_and_noble.random_middle(),
			self.fair_and_noble.random_male_suffix()
		)
	}

	pub fn elf_female_name(&self) -> String {
		format!(
			"{}{}{}",
			self.fair_and_noble.random_elf_prefix(),
			self.fair_and_noble.random_middle(),
			self.fair_and_noble.random_female_suffix()
		)
	}

	pub fn elf_male_name_alternative(&self) -> String {
		format!(
			"{}{}{}",
			self.fair_and_noble.random_elf_prefix_alternative(),
			self.fair_and_noble.random_middle(),
			self.fair_and_noble.random_male_suffix()
		)
	}

	pub fn elf_female_name_alternative(&self) -> String {
		format!(
			"{}{}{}",
			self.fair_and_noble.random_elf_prefix_alternative(),
			self.fair_and_noble.random_middle(),
			self.fair_and_noble.random_female_suffix()
		)
	}

	pub fn faery_male_name(&self) -> String {
		format!(
			"{}{}",
			self.faery_kind.random_prefix(),
			self.faery_kind.random_male_suffix()
		)
	}

	pub fn faery_female_name(&self) -> String {
		format!(
			"{}{}",
			self.faery_kind.random_prefix(),
			self.faery_kind.random_female_suffix()
		)
	}

	pub fn faery_male_name_alternative(&self) -> String {
		format!(
			"{}{}",
			self.alternative_faery_kind.random_prefix(),
			self.alternative_faery_kind.random_male_suffix()
		)
	}

	pub fn faery_female_name_alternative(&self) -> String {
		format!(
			"{}{}",
			self.alternative_faery_kind.random_prefix(),
			self.alternative_faery_kind.random_female_suffix()
		)
	}

	pub fn dark_elf_male_name(&self) -> String {
		let mut rng = thread_rng();
		let mut name = self.elegant_evil.random_dark_elf_prefix();
		let d100 = rng.gen_range(0, 100);
		if d100 > 17 {
			name += &self.elegant_evil.random_middle();
		}
		name += &self.elegant_evil.random_male_suffix();
		name
	}

	pub fn dark_elf_female_name(&self) -> String {
		let mut rng = thread_rng();
		let mut name = self.elegant_evil.random_dark_elf_prefix();
		let d100 = rng.gen_range(0, 100);
		if d100 > 17 {
			name += &self.elegant_evil.random_middle();
		}
		name += &self.elegant_evil.random_female_suffix();
		name
	}

	pub fn dark_elf_male_name_alternative(&self) -> String {
		let mut rng = thread_rng();
		let mut name = self.elegant_evil.random_dark_elf_prefix_alternative();
		let d100 = rng.gen_range(0, 100);
		if d100 > 17 {
			name += &self.elegant_evil.random_middle();
		}
		name += &self.elegant_evil.random_male_suffix();
		name
	}

	pub fn dark_elf_female_name_alternative(&self) -> String {
		let mut rng = thread_rng();
		let mut name = self.elegant_evil.random_dark_elf_prefix_alternative();
		let d100 = rng.gen_range(0, 100);
		if d100 > 17 {
			name += &self.elegant_evil.random_middle();
		}
		name += &self.elegant_evil.random_female_suffix();
		name
	}

	pub fn half_demon_male_name(&self) -> String {
		format!(
			"{}{}",
			self.malevolent.random_prefix(),
			self.malevolent.random_male_suffix()
		)
	}

	pub fn half_demon_female_name(&self) -> String {
		format!(
			"{}{}",
			self.malevolent.random_prefix(),
			self.malevolent.random_female_suffix()
		)
	}

	pub fn dragon_male_name(&self) -> String {
		format!(
			"{}{}",
			self.draconic.random_prefix(),
			self.draconic.random_suffix()
		)
	}

	pub fn dragon_female_name(&self) -> String {
		let name = self.draconic.random_prefix();
		let mut suffix = self.draconic.random_suffix();
		if suffix == "bazius" {
			suffix = "bazia".to_string();
		} else if suffix.ends_with("os") {
			suffix = suffix.chars().take(suffix.len() - 2).collect();
			suffix += &"ossa".to_string();
		} else {
			suffix += &"is".to_string();
		}
		format!("{}{}", name, suffix)
	}

	pub fn demon_name(&self) -> String {
		let mut rng = thread_rng();
		let d6 = rng.gen_range(0, 6);
		match d6 {
			0 => format!(
				"{}{}",
				self.infernal.random_soft(),
				self.infernal.random_dull()
			),
			1 => format!(
				"{}{}",
				self.infernal.random_soft(),
				self.infernal.random_sharp()
			),
			2 => format!(
				"{}{}",
				self.infernal.random_dull(),
				self.infernal.random_soft()
			),
			3 => format!(
				"{}{}",
				self.infernal.random_dull(),
				self.infernal.random_sharp()
			),
			4 => format!(
				"{}{}",
				self.infernal.random_sharp(),
				self.infernal.random_soft()
			),
			_ => format!(
				"{}{}",
				self.infernal.random_sharp(),
				self.infernal.random_dull()
			),
		}
	}

	pub fn angel_male_name(&self) -> String {
		let mut name = self.empyreal.random_prefix();
		let mut rng = thread_rng();
		let d100 = rng.gen_range(0, 100);
		if d100 <= 8 {
			if !name.ends_with("ar") && !name.ends_with("al") {
				name = format!("{}{}", self.empyreal.random_title(), name);
			} else if name.ends_with("ar") {
				name = format!("{}{}", self.empyreal.random_title_al(), name);
			} else {
				name = format!("{}{}", self.empyreal.random_title_ar(), name);
			}
		} else {
			name += &self.empyreal.random_male_suffix();
		}

		name
	}

	pub fn angel_female_name(&self) -> String {
		let mut name = self.empyreal.random_prefix();
		let mut rng = thread_rng();
		let d100 = rng.gen_range(0, 100);
		if d100 <= 8 {
			if let Some(a_index) = name.rfind('a') {
				// Replace the last 'a' by a 'e' for female
				let ending: String = name.chars().skip(a_index + 1).collect();
				name = name.chars().take(a_index).collect();
				name = format!("{}{}", name, ending);
			}
			if !name.ends_with("ar") && !name.ends_with("al") {
				name = format!("{}{}", self.empyreal.random_title(), name);
			} else if name.ends_with("ar") {
				name = format!("{}{}", self.empyreal.random_title_al(), name);
			} else {
				name = format!("{}{}", self.empyreal.random_title_ar(), name);
			}
		} else {
			name += &self.empyreal.random_female_suffix();
		}

		name
	}
}

use grue::generation::names::*;
use heck::TitleCase;
use regex::{Captures, Regex};

pub fn load_military_unit() -> MilitaryUnit {
	let defaults = include_str!("../data/military_unit.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default MilitaryUnit config: {:?}", err);
			MilitaryUnit {
				..Default::default()
			}
		}
	}
}

pub fn load_mystic_order() -> MysticOrder {
	let defaults = include_str!("../data/mystic_order.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default MysticOrder config: {:?}", err);
			MysticOrder {
				..Default::default()
			}
		}
	}
}

pub fn load_thieves_assassins() -> ThievesAssassins {
	let defaults = include_str!("../data/thieves_assassins.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default ThievesAssassins config: {:?}", err);
			ThievesAssassins {
				..Default::default()
			}
		}
	}
}

pub fn load_generic_fantasy() -> GenericFantasy {
	let defaults = include_str!("../data/generic_fantasy.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default GenericFantasy config: {:?}", err);
			GenericFantasy {
				..Default::default()
			}
		}
	}
}

pub fn load_specific_fantasy() -> SpecificFantasy {
	let defaults = include_str!("../data/specific_fantasy.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default SpecificFantasy config: {:?}", err);
			SpecificFantasy {
				..Default::default()
			}
		}
	}
}

pub fn load_tavern() -> Tavern {
	let defaults = include_str!("../data/tavern.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default Tavern config: {:?}", err);
			Tavern {
				..Default::default()
			}
		}
	}
}

fn main() {
	let mystic_order = load_mystic_order();

	let pattern = mystic_order.random_pattern();
	//println!("pattern: {}", pattern);

	let re = Regex::new(r"<([\w\W]*?)>").unwrap();
	let mystic = re.replace_all(&pattern, |caps: &Captures| {
		let token = &caps[0];
		match token {
			"<group>" => mystic_order.random_group(),
			"<description>" => mystic_order.random_descriptive(),
			"<entity>" => mystic_order.random_entity(),
			_ => {
				// Unknown match, return original
				token.to_string()
			}
		}
	});

	println!("mystic order: {}", mystic.to_title_case());

	let generic_fantasy = load_generic_fantasy();
	println!(
		"generic fantasy name: {}",
		generic_fantasy.random_name().to_title_case()
	);

	let tavern = load_tavern();
	println!(
		"tavern name: {}",
		tavern.random_name("", "", "", "").to_title_case()
	);

	let military_unit = load_military_unit();
	let pattern2 = military_unit.random_pattern();
	let military = re.replace_all(&pattern2, |caps: &Captures| {
		let token = &caps[0];
		match token {
			// TODO: Track returned substitutions so no duplicates are ever returned (i.e. <description> <description>)
			"<group>" => military_unit.random_group(),
			"<description>" => military_unit.random_descriptive(),
			"<location>" => military_unit.random_location(),
			"<commander>" => generic_fantasy.random_name(),
			_ => {
				// Unknown match, return original
				token.to_string()
			}
		}
	});

	println!("military unit: {}", military.to_title_case());

	let thieves_assassins = load_thieves_assassins();
	println!(
		"thieves and assassins: {}",
		thieves_assassins.random_name().to_title_case()
	);

	let specific_fantasy = load_specific_fantasy();
	println!("goblin: {}", specific_fantasy.goblin_name());
	println!("orc: {}", specific_fantasy.orc_name());
	println!("ogre: {}", specific_fantasy.ogre_name());
	println!("caveman [male]: {}", specific_fantasy.caveman_male_name());
	println!(
		"caveman [female]: {}",
		specific_fantasy.caveman_female_name()
	);
	println!("dwarf [male]: {}", specific_fantasy.dwarf_male_name());
	println!("dwarf [female]: {}", specific_fantasy.dwarf_female_name());
	println!("halfling [male]: {}", specific_fantasy.halfling_male_name());
	println!(
		"halfling [female]: {}",
		specific_fantasy.halfling_female_name()
	);
	println!("gnome [male]: {}", specific_fantasy.gnome_male_name());
	println!("gnome [female]: {}", specific_fantasy.gnome_female_name());
	println!("elf [male]: {}", specific_fantasy.elf_male_name());
	println!("elf [female]: {}", specific_fantasy.elf_female_name());
	println!(
		"elf alternative [male]: {}",
		specific_fantasy.elf_male_name_alternative()
	);
	println!(
		"elf alternative [female]: {}",
		specific_fantasy.elf_female_name_alternative()
	);
	println!("dark elf [male]: {}", specific_fantasy.dark_elf_male_name());
	println!(
		"dark elf [female]: {}",
		specific_fantasy.dark_elf_female_name()
	);
	println!(
		"dark elf alternative [male]: {}",
		specific_fantasy.dark_elf_male_name_alternative()
	);
	println!(
		"dark elf alternative [female]: {}",
		specific_fantasy.dark_elf_female_name_alternative()
	);
	println!("faery [male]: {}", specific_fantasy.faery_male_name());
	println!("faery [female]: {}", specific_fantasy.faery_female_name());
	println!(
		"faery alternative [male]: {}",
		specific_fantasy.faery_male_name_alternative()
	);
	println!(
		"faery alternative [female]: {}",
		specific_fantasy.faery_female_name_alternative()
	);
	println!(
		"half demon [male]: {}",
		specific_fantasy.half_demon_male_name()
	);
	println!(
		"half demon [female]: {}",
		specific_fantasy.half_demon_female_name()
	);
	println!("dragon [male]: {}", specific_fantasy.dragon_male_name());
	println!("dragon [female]: {}", specific_fantasy.dragon_female_name());
	println!("demon: {}", specific_fantasy.demon_name());
	println!("angel [male]: {}", specific_fantasy.angel_male_name());
	println!("angel [female]: {}", specific_fantasy.angel_female_name());
}

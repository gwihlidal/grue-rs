use grue::generation::adventure::*;

pub fn load_adventures() -> Adventures {
	let defaults = include_str!("../data/adventures.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default Adventures config: {:?}", err);
			Adventures {
				..Default::default()
			}
		}
	}
}

fn main() {
	let adventures = load_adventures();

	println!("Random Adventure");
	println!("==");
	println!();

	let theme = adventures.random_theme();
	println!("Theme: {}", theme.0);
	println!("{}", theme.1);
	println!();

	let goal = adventures.random_goal();
	println!("Goal: {}", goal.0);
	println!("{}", goal.1);
	println!();

	let hook = adventures.random_story_hook();
	println!("Hook: {}", hook.0);
	println!("{}", hook.1);
	println!();

	let plot = adventures.random_plot();
	println!("Plot: {}", plot.0);
	println!("{}", plot.1);
	println!();

	let climax = adventures.random_climax();
	println!("Climax: {}", climax.0);
	println!("{}", climax.1);
	println!();

	let general_setting = adventures.random_general_setting();
	println!("General Setting: {}", general_setting.0);
	println!("{}", general_setting.1);
	println!();

	let specific_setting_1 = adventures.random_specific_setting();
	println!("Specific Setting 1: {}", specific_setting_1.0);
	println!("{}", specific_setting_1.1);
	println!();

	let specific_setting_2 = adventures.random_specific_setting();
	println!("Specific Setting 2: {}", specific_setting_2.0);
	println!("{}", specific_setting_2.1);
	println!();

	let master_villain = adventures.random_master_villain();
	println!("Master Villain: {}", master_villain.0);
	println!("{}", master_villain.1);
	println!();

	let minor_villain_1 = adventures.random_minor_villain();
	println!("Minor Villain 1: {}", minor_villain_1.0);
	println!("{}", minor_villain_1.1);
	println!();

	let minor_villain_2 = adventures.random_minor_villain();
	println!("Minor Villain 2: {}", minor_villain_2.0);
	println!("{}", minor_villain_2.1);
	println!();

	let ally_neutral = adventures.random_ally_neutral();
	println!("Ally/Neutral: {}", ally_neutral.0);
	println!("{}", ally_neutral.1);
	println!();

	let monster_encounter = adventures.random_monster_encounter();
	println!("Monster Encounter: {}", monster_encounter.0);
	println!("{}", monster_encounter.1);
	println!();

	let character_encounter = adventures.random_character_encounter();
	println!("Character Encounter: {}", character_encounter.0);
	println!("{}", character_encounter.1);
	println!();

	let death_trap = adventures.random_death_trap();
	println!("Death Trap: {}", death_trap.0);
	println!("{}", death_trap.1);
	println!();

	let chase = adventures.random_chase();
	println!("Chase: {}", chase.0);
	println!("{}", chase.1);
	println!();

	let omen_prophecy = adventures.random_omen_prophecy();
	println!("Omen/Prophecy: {}", omen_prophecy.0);
	println!("{}", omen_prophecy.1);
	println!();

	let secret_weakness = adventures.random_secret_weakness();
	println!("Secret Weakness: {}", secret_weakness.0);
	println!("{}", secret_weakness.1);
	println!();

	let special_condition = adventures.random_special_condition();
	println!("Special Condition: {}", special_condition.0);
	println!("{}", special_condition.1);
	println!();

	let moral_quandary = adventures.random_moral_quandary();
	println!("Moral Quandary: {}", moral_quandary.0);
	println!("{}", moral_quandary.1);
	println!();

	let red_herring = adventures.random_red_herring();
	println!("Red Herring: {}", red_herring.0);
	println!("{}", red_herring.1);
	println!();

	let cruel_trick = adventures.random_cruel_trick();
	println!("Cruel Trick: {}", cruel_trick.0);
	println!("{}", cruel_trick.1);
	println!();
}

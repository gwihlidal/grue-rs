use grue::generation::markov::*;
use heck::TitleCase;

pub fn get_markov_data() -> MarkovDataGroup {
	let defaults = include_str!("../data/markov_chain.toml");
	match toml::from_str(defaults) {
		Ok(result) => result,
		Err(err) => {
			println!("failed to load default Markov data: {:?}", err);
			MarkovDataGroup {
				..Default::default()
			}
		}
	}
}

fn main() {
	let result_count = 5;

	let markov_order = 3;
	let markov_data = get_markov_data();

	let config = MarkovGenerateConfig {
		min_len: 4,
		max_len: 13,
		starts_with: "",
		ends_with: "",
		contains: "",
		does_not_contain: "",
		allow_duplicates: false,
		max_attempts: 500,
	};

	for dataset in &markov_data.datasets {
		println!("\nData Set: {}", dataset.name);

		let values: Vec<&str> = dataset.values.iter().map(|x| x.as_ref()).collect();

		let mut markov_gen = MarkovNameGenerator::new(markov_order);
		markov_gen.build_chain(&values);

		for x in 0..result_count {
			let result = markov_gen.generate_word(&config);
			println!("result [{}] is {}", x, result.to_title_case());
		}
	}
}

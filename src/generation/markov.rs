use indextree::Arena;
use indextree::NodeId;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MarkovDataSet {
	pub name: String,
	pub values: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MarkovDataGroup {
	pub datasets: Vec<MarkovDataSet>,
}

#[derive(Default)]
pub struct MarkovNode {
	pub ch: char,
	pub neighbours: Vec<NodeId>,
}

impl MarkovNode {
	pub fn new(ch: char) -> MarkovNode {
		MarkovNode {
			ch,
			neighbours: vec![],
		}
	}
}

pub struct MarkovGenerateConfig<'a> {
	pub min_len: i32,
	pub max_len: i32,
	pub starts_with: &'a str,
	pub ends_with: &'a str,
	pub contains: &'a str,
	pub does_not_contain: &'a str,
	pub allow_duplicates: bool,
	pub max_attempts: u32,
}

pub struct MarkovNameGenerator {
	pub order: usize,
	pub arena: Arena<MarkovNode>,
	pub root_id: NodeId,
	pub duplicates_id: NodeId,
	pub invalid_id: NodeId,
	pub node_map: HashMap<String, NodeId>,
}

impl MarkovNameGenerator {
	pub fn new(order: usize) -> MarkovNameGenerator {
		let mut arena = Arena::<MarkovNode>::new();
		let root_id = arena.new_node(MarkovNode::new(' '));
		let duplicates_id = arena.new_node(MarkovNode::new(' '));
		let invalid_id = arena.new_node(MarkovNode::new(' '));
		MarkovNameGenerator {
			order,
			arena,
			root_id,
			duplicates_id,
			invalid_id,
			node_map: HashMap::default(),
		}
	}

	pub fn build_chain(&mut self, tokens: &[&str]) {
		for token in tokens {
			self.add_token(&token);
		}
	}

	fn add_token(&mut self, token: &str) {
		let mut previous_id = self.root_id;
		let mut key = String::default();
		for ch in token.chars().into_iter() {
			key.push(ch);
			if key.len() > self.order {
				key = key.chars().skip(1).collect();
			}

			let new_node_id = if self.node_map.contains_key(&key) {
				*self.node_map.get(&key).unwrap()
			} else {
				let node_id = self.arena.new_node(MarkovNode::new(ch));
				self.node_map.insert(key.to_owned(), node_id);
				node_id
			};

			if previous_id != new_node_id {
				previous_id.append(new_node_id, &mut self.arena).unwrap();
			}

			self.arena
				.get_mut(previous_id)
				.unwrap()
				.data
				.neighbours
				.push(new_node_id);
			previous_id = new_node_id;
		}

		let duplicates = self.duplicates_id;
		self.add_duplicate(&token.to_lowercase(), duplicates);
		self.add_duplicate(&token.to_lowercase(), duplicates);
		self.arena
			.get_mut(previous_id)
			.unwrap()
			.data
			.neighbours
			.push(self.invalid_id);
	}

	fn add_duplicate(&mut self, token: &str, duplicates: NodeId) {
		if token.len() > 1 {
			let sub_token: String = token.chars().skip(1).collect();
			self.add_duplicate(&sub_token, duplicates);
		}

		let mut current_node_id = self.duplicates_id;
		for ch in token.chars().into_iter() {
			let previous_node_id = current_node_id;

			for child_node_id in &self.arena.get(current_node_id).unwrap().data.neighbours {
				let child_node = &self.arena.get(*child_node_id).unwrap().data;
				if child_node.ch == ch {
					current_node_id = *child_node_id;
					break;
				}
			}

			if current_node_id == previous_node_id {
				let new_node_id = &self.arena.new_node(MarkovNode::new(ch));
				self.arena
					.get_mut(current_node_id)
					.unwrap()
					.data
					.neighbours
					.push(new_node_id.clone());
				current_node_id = *new_node_id;
			}
		}
	}

	fn is_duplicate(&self, token: &str) -> bool {
		let token_lower = token.to_lowercase();
		let mut current_node_id = self.duplicates_id;
		for ch in token_lower.chars().into_iter() {
			let current_node = &self.arena.get(current_node_id).unwrap().data;

			let previous_node_id = current_node_id;

			for child_node_id in &current_node.neighbours {
				let child_node = &self.arena.get(*child_node_id).unwrap().data;
				if child_node.ch == ch {
					current_node_id = *child_node_id;
					break;
				}
			}

			if current_node_id == previous_node_id {
				return false;
			}
		}

		true
	}

	pub fn generate_word(&self, config: &MarkovGenerateConfig) -> String {
		let safe_min_len = if config.min_len <= 0
			|| (config.min_len as usize) < config.starts_with.len()
			|| (config.min_len as usize) < config.ends_with.len()
		{
			if config.starts_with.len() > config.ends_with.len() {
				config.starts_with.len()
			} else {
				config.ends_with.len()
			}
		} else {
			config.min_len as usize
		};

		let safe_max_len = if config.max_len < 0 {
			256usize
		} else {
			config.max_len as usize
		};

		let safe_max_attempts = if config.max_attempts == 0 {
			100
		} else {
			config.max_attempts
		};

		let mut attempts = 0;
		let mut word = String::default();
		let mut rng = thread_rng();

		while word.is_empty() {
			attempts += 1;
			if attempts >= safe_max_attempts {
				println!("too many attempts! aborting name generation");
				break;
			}

			let root_node = &self.arena.get(self.root_id).unwrap().data;
			let mut next_node_index = rng.gen_range(0, root_node.neighbours.len());
			let mut current_node_id = root_node.neighbours[next_node_index];
			word.clear();

			while current_node_id != self.invalid_id && word.len() <= safe_max_len {
				let current_node = &self.arena.get(current_node_id).unwrap().data;
				word.push(current_node.ch);
				next_node_index = rng.gen_range(0, current_node.neighbours.len());
				current_node_id = current_node.neighbours[next_node_index];
			}

			// Verify result against a variety of invalid checks
			let check1 = !config.starts_with.is_empty() && !word.starts_with(config.starts_with);
			let check2 = !config.ends_with.is_empty() && !word.ends_with(config.ends_with);
			let check3 = !config.contains.is_empty() && !word.contains(config.contains);
			let check4 =
				!config.does_not_contain.is_empty() && word.contains(config.does_not_contain);
			let check5 = word.len() > safe_max_len || word.len() < safe_min_len;
			let check6 = !config.allow_duplicates && self.is_duplicate(&word);
			let invalid = check1 || check2 || check3 || check4 || check5 || check6;
			if invalid {
				word.clear();
			}
		}

		word
	}
}

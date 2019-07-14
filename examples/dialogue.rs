use grue::dialogue::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

struct Evaluator {
	variables: RefCell<HashMap<VariableKey, VariableVal>>,
}

impl NodeEvents for Evaluator {
	fn on_visit(&self, _dlg: &Dialogue, _node_ref: NodeRef) {
		//println!("on_visit: ");
	}

	fn on_text(&self, _dlg: &Dialogue, _id: NodeId, text: &str) {
		println!("** {}", text);
	}

	fn on_choice(
		&self,
		dlg: &Dialogue,
		_node_ref: NodeRef,
		text: &str,
		choices: &[NodeId],
	) -> Option<NodeId> {
		println!("** {}", text);
		for choice in 0..choices.len() {
			let choice_node = dlg.get_node(choices[choice]).expect("invalid node index");
			let choice_data = &choice_node.data;
			match choice_data.variant {
				NodeVariant::Text((_, ref text)) => {
					println!("   {} - {}", choice, text);
				}
				_ => {}
			}
		}

		stdout().write(b"Input: ").unwrap();
		let _ = stdout().flush();

		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.expect("unable to read user input");

		let selected = input.trim().parse::<usize>().expect("not a valid choice");
		if selected < choices.len() {
			let selected_node = dlg.get_node(choices[selected]).expect("invalid node index");
			match selected_node.data.variant {
				NodeVariant::Text((next, _)) => next,
				_ => None,
			}
		} else {
			None
		}
	}
	fn on_set(&self, _dlg: &Dialogue, key: &str, value: &str) {
		let mut vars = self.variables.borrow_mut();
		let var = vars.entry(key.to_string()).or_insert(value.to_string());
		*var = value.to_string();
		println!("[Setting '{}' to '{}']", key, *var);
	}
	fn on_get(&self, _dlg: &Dialogue, key: &str) -> Option<String> {
		let vars = self.variables.borrow();
		match vars.get(key) {
			Some(ref val) => {
				println!("[Value of '{}' is '{}']", key, val);
				Some(val.to_owned().to_string())
			}
			None => None,
		}
	}
}

fn execute(dialogue: &Dialogue, node_id: Option<NodeId>) {
	if let Some(node_id) = node_id {
		let next = dialogue.step(node_id);
		execute(dialogue, next);
	}
}

fn main() {
	let mut dialogue = Dialogue::new();
	dialogue.set_events(Evaluator {
		variables: RefCell::new(HashMap::new()),
	});

	dialogue.insert_node(Node::new_text(
		0,
		"Text 0",
		Some(1),
		"This is the first line",
	));
	dialogue.insert_node(Node::new_text(
		1,
		"Text 1",
		Some(2),
		"This is the second line",
	));
	dialogue.insert_node(Node::new_text(
		2,
		"Text 2",
		Some(3),
		"This is the third line",
	));

	dialogue.insert_node(Node::new_choice(
		3,
		"Choice 0",
		"What is your favourite colour?",
		&[4, 5, 6],
	));

	dialogue.insert_node(Node::new_text(4, "Choice_R", Some(7), "Red"));
	dialogue.insert_node(Node::new_text(5, "Choice_G", Some(8), "Green"));
	dialogue.insert_node(Node::new_text(6, "Choice_B", Some(9), "Blue"));

	dialogue.insert_node(Node::new_set(
		7,
		"Set_R",
		Some(10),
		&"colour_choice".to_string(),
		&"red".to_string(),
	));
	dialogue.insert_node(Node::new_set(
		8,
		"Set_G",
		Some(10),
		&"colour_choice".to_string(),
		&"green".to_string(),
	));
	dialogue.insert_node(Node::new_set(
		9,
		"Set_B",
		Some(10),
		&"colour_choice".to_string(),
		&"blue".to_string(),
	));

	let colour_branches = &[("red", 11), ("green", 12), ("blue", 13)];
	dialogue.insert_node(Node::new_branch(
		10,
		"Colour_Branch",
		&"colour_choice".to_string(),
		colour_branches,
	));

	dialogue.insert_node(Node::new_text(
		11,
		"Chose_Red",
		Some(14),
		"Oh, red is your favourite colour!",
	));
	dialogue.insert_node(Node::new_text(
		12,
		"Chose_Green",
		Some(14),
		"Oh, green is your favourite colour!",
	));
	dialogue.insert_node(Node::new_text(
		13,
		"Chose_Blue",
		Some(14),
		"Oh, blue is your favourite colour!",
	));

	dialogue.insert_node(Node::new_text(14, "Ending", None, "Bye!"));

	let graph = dialogue.export();
	let yaml = graph.export();
	println!("YAML:\n{}", yaml);

	println!("\n\nStart dialogue:\n");
	execute(&dialogue, Some(0));
}

use indextree::Arena;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeId = u32;
pub type NodeRef<'a> = &'a indextree::Node<Node>;
pub type VariableKey = String;
pub type VariableVal = String;

#[allow(unused_variables)]
pub trait NodeEvents {
	fn on_visit(&self, dlg: &Dialogue, node_ref: NodeRef) {}
	fn on_text(&self, dlg: &Dialogue, id: NodeId, text: &str) {}
	fn on_choice(
		&self,
		dlg: &Dialogue,
		node_ref: NodeRef,
		text: &str,
		choices: &[NodeId],
	) -> Option<NodeId> {
		None
	}
	fn on_set(&self, dlg: &Dialogue, key: &str, value: &str) {}
	fn on_get(&self, dlg: &Dialogue, key: &str) -> Option<String> {
		None
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeVariant {
	Text((Option<NodeId>, String)),
	Choice((String, Vec<NodeId>)),
	Set((Option<NodeId>, VariableKey, VariableVal)),
	Branch((VariableKey, HashMap<VariableVal, NodeId>)),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
	pub id: NodeId,
	pub label: String,
	pub variant: NodeVariant,
}

impl Node {
	pub fn new_text(id: NodeId, label: &str, next: Option<NodeId>, text: &str) -> Node {
		Node {
			id,
			label: label.to_owned(),
			variant: NodeVariant::Text((next, text.to_owned())),
		}
	}

	pub fn new_choice(id: NodeId, label: &str, text: &str, choices: &[NodeId]) -> Node {
		Node {
			id,
			label: label.to_owned(),
			variant: NodeVariant::Choice((text.to_owned(), choices.to_owned())),
		}
	}

	pub fn new_set(id: NodeId, label: &str, next: Option<NodeId>, key: &str, val: &str) -> Node {
		Node {
			id,
			label: label.to_owned(),
			variant: NodeVariant::Set((next, key.to_owned(), val.to_owned())),
		}
	}

	pub fn new_branch(id: NodeId, label: &str, key: &str, branches: &[(&str, NodeId)]) -> Node {
		let mut mapping = HashMap::new();
		for branch in branches {
			mapping.insert(branch.0.to_owned(), branch.1);
		}
		Node {
			id,
			label: label.to_owned(),
			variant: NodeVariant::Branch((key.to_owned(), mapping)),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeGraph {
	pub nodes: Vec<Node>,
}

impl NodeGraph {
	pub fn export(&self) -> String {
		serde_yaml::to_string(&self.nodes).expect("failed to serialize node graph")
	}
}

pub struct Dialogue {
	node_arena: Arena<Node>,
	node_lookup: HashMap<NodeId, indextree::NodeId>,
	events: Option<Box<dyn NodeEvents>>,
}

impl Default for Dialogue {
	fn default() -> Self {
		Self::new()
	}
}

impl Dialogue {
	pub fn new() -> Self {
		Dialogue {
			node_arena: Arena::new(),
			node_lookup: HashMap::new(),
			events: None,
		}
	}

	pub fn insert_node(&mut self, node: Node) {
		let node_id = node.id;
		let arena_id = self.node_arena.new_node(node);
		self.node_lookup.insert(node_id, arena_id);
	}

	pub fn remove_node(&mut self, _node: NodeId) {}

	pub fn get_node(&self, id: NodeId) -> Option<NodeRef> {
		match self.node_lookup.get(&id) {
			Some(node_id) => match self.node_arena.get(*node_id) {
				Some(node) => Some(node),
				None => None,
			},
			None => None,
		}
	}

	pub fn set_events<E: NodeEvents + 'static>(&mut self, events: E) {
		self.events = Some(Box::new(events));
	}

	pub fn validate(&self) {
		for _node in self.node_arena.iter() {
			//
		}
	}

	pub fn step(&self, id: NodeId) -> Option<NodeId> {
		match self.get_node(id) {
			Some(ref node_ref) => {
				if let Some(ref events) = self.events {
					events.on_visit(self, node_ref);
				}
				match node_ref.data.variant {
					NodeVariant::Text((next, ref text)) => {
						if let Some(ref events) = self.events {
							events.on_text(self, node_ref.data.id, text);
						}
						next
					}
					NodeVariant::Choice((ref text, ref choices)) => {
						if let Some(ref events) = self.events {
							events.on_choice(self, node_ref, &text, &choices)
						} else {
							None
						}
					}
					NodeVariant::Set((next, ref key, ref val)) => {
						if let Some(ref events) = self.events {
							events.on_set(self, &key, &val);
						}
						next
					}
					NodeVariant::Branch((ref key, ref branches)) => {
						if let Some(ref events) = self.events {
							let value = events.on_get(self, &key);
							match value {
								Some(value) => match branches.get(&value) {
									Some(node_id) => Some(*node_id),
									None => None,
								},
								None => None,
							}
						} else {
							None
						}
					}
				}
			}
			None => None,
		}
	}

	pub fn export(&self) -> NodeGraph {
		NodeGraph {
			nodes: self.node_arena.iter().map(|x| x.data.clone()).collect(),
		}
	}
}

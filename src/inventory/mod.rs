use std::collections::HashMap;
use uuid::Uuid;

pub type ItemId = Uuid;
pub type InstanceId = Uuid;

pub enum Rarity {
	Common,
	Uncommon,
	Rare,
	VeryRare,
	Legendary,
}

pub enum Type {
	Armor,
	Potion,
	Ring,
	Rod,
	Scroll,
	Staff,
	Wand,
	Weapon,
	Magic,
}

pub struct Limits {
	pub stacking: u32,
	pub possesion: u32,
	pub equipped: u32,
}

pub struct Item {
	pub id: ItemId,
	pub name: String,
	pub desc: String,
	pub value: u32,
	pub weight: u32,
	pub rarity: Rarity,
	pub limits: Limits,
}

impl Item {
	pub fn new(
		name: &str,
		desc: &str,
		value: u32,
		weight: u32,
		rarity: Rarity,
		limits: Limits,
	) -> Item {
		Item {
			id: Uuid::new_v4(),
			name: name.to_string(),
			desc: desc.to_string(),
			value,
			weight,
			rarity,
			limits,
		}
	}
}

pub struct ItemInstance {
	pub id: InstanceId,
	pub item: ItemId,
	pub quantity: u32,
}

impl ItemInstance {
	pub fn new(item: ItemId, quantity: u32) -> ItemInstance {
		ItemInstance {
			id: Uuid::new_v4(),
			item,
			quantity,
		}
	}
}

pub struct ItemDatabase {
	pub items: HashMap<ItemId, Item>,
}

pub struct Inventory {
	pub currency: u32,
	pub instances: HashMap<InstanceId, ItemInstance>,
	pub database: ItemDatabase,
}

impl Inventory {
	pub fn new(database: ItemDatabase) -> Inventory {
		Inventory {
			currency: 0,
			instances: HashMap::new(),
			database,
		}
	}

	pub fn get_currency(&self) -> u32 {
		self.currency
	}

	pub fn set_currency(&mut self, currency: u32) {
		self.currency = currency;
	}

	pub fn instances_by_item(&self, id: ItemId) -> Vec<InstanceId> {
		self.instances
			.iter()
			.filter(|(_, instance)| instance.item == id)
			.map(|(_, instance)| instance.id)
			.collect()
	}

	pub fn get_instance(&self, id: InstanceId) -> Option<&ItemInstance> {
		self.instances.get(&id)
	}

	pub fn add_instance(&mut self, instance: ItemInstance) {
		if let Some(_item) = self.database.items.get(&instance.item) {
			self.instances.insert(instance.id, instance);
			//let stacking = item.limits.stacking;
			/*let instances = self.instances_by_item(instance.item);
			for inst in &instances {
				if let Some(mut existing_instance) = self.instances.get_mut(&inst) {
					if existing_instance.quantity + instance.quantity <= stacking {
						existing_instance.quantity += instance.quantity;
						return;
					}
				}
			}*/
		}
	}

	pub fn remove_instance(&mut self, id: InstanceId) {
		if let Some(ref _item) = self.instances.remove(&id) {
			//
		}
	}

	pub fn total_weight(&self) -> u32 {
		self.instances
			.iter()
			.fold(0, |total_weight, (_, instance)| {
				let weight = if let Some(item) = self.database.items.get(&instance.item) {
					item.weight
				} else {
					0
				};

				total_weight + (weight * instance.quantity)
			})
	}

	pub fn total_value(&self) -> u32 {
		self.instances.iter().fold(0, |total_value, (_, instance)| {
			let value = if let Some(item) = self.database.items.get(&instance.item) {
				item.value
			} else {
				0
			};

			total_value + (value * instance.quantity)
		})
	}
}

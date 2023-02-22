use valence::{
	prelude::*,
	protocol::{packets::s2c::play::SetContainerSlot, VarInt},
};

#[derive(Component, Default)]
pub struct Level(u8);

impl Level {
	pub fn increase(&mut self, client: &mut Mut<Client>, equipment: &Res<EquipmentLevel>) {
		self.0 += 1;
		equip(self, client, equipment);
	}
	pub fn decrease(&mut self, client: &mut Mut<Client>, equipment: &Res<EquipmentLevel>) {
		self.0 /= 2;
		equip(self, client, equipment);
	}
}

fn equip(level: &Level, client: &mut Mut<Client>, equipment: &Res<EquipmentLevel>) {
	if let Some(equipment) = equipment.get(level) {
		set_slot(client, 0, equipment.weapon);
		if let Some(chest) = equipment.chest {
			set_slot(client, 38, chest);
		}
		if let Some(legs) = equipment.legs {
			set_slot(client, 37, legs);
		}
		if let Some(head) = equipment.head {
			set_slot(client, 39, head);
		}
		if let Some(feet) = equipment.feet {
			set_slot(client, 36, feet);
		}
	}
}

fn set_slot(client: &mut Mut<Client>, id: i16, item: ItemKind) {
	let item_stack = ItemStack::new(item, 1, None);
	client.write_packet(&SetContainerSlot {
		window_id: -2,
		state_id: VarInt(0),
		slot_idx: id,
		slot_data: Some(item_stack),
	});
}

#[derive(Resource)]
pub struct EquipmentLevel {
	pub equipments: Vec<Equipment>,
}

impl EquipmentLevel {
	pub fn new() -> Self {
		let mut equipments: Vec<Equipment> = Vec::new();
		equipments.push(EquipmentBuilder::new(ItemKind::WoodenSword).build());
		equipments.push(
			EquipmentBuilder::new(ItemKind::WoodenSword)
				.chest(ItemKind::LeatherChestplate)
				.build(),
		);
		equipments.push(
			EquipmentBuilder::new(ItemKind::WoodenSword)
				.chest(ItemKind::LeatherChestplate)
				.legs(ItemKind::LeatherLeggings)
				.build(),
		);
		equipments.push(
			EquipmentBuilder::new(ItemKind::WoodenSword)
				.chest(ItemKind::LeatherChestplate)
				.legs(ItemKind::LeatherLeggings)
				.head(ItemKind::LeatherHelmet)
				.build(),
		);
		equipments.push(
			EquipmentBuilder::new(ItemKind::WoodenSword)
				.chest(ItemKind::LeatherChestplate)
				.legs(ItemKind::LeatherLeggings)
				.head(ItemKind::LeatherHelmet)
				.feet(ItemKind::LeatherBoots)
				.build(),
		);
		equipments.push(
			EquipmentBuilder::new(ItemKind::Trident)
				.head(ItemKind::TurtleHelmet)
				.build(),
		);
		Self { equipments }
	}
	fn get(&self, level: &Level) -> Option<&Equipment> {
		self.equipments.get(level.0 as usize)
	}
}

pub struct Equipment {
	pub weapon: ItemKind,
	pub head: Option<ItemKind>,
	pub chest: Option<ItemKind>,
	pub legs: Option<ItemKind>,
	pub feet: Option<ItemKind>,
}

struct EquipmentBuilder {
	weapon: ItemKind,
	head: Option<ItemKind>,
	chest: Option<ItemKind>,
	legs: Option<ItemKind>,
	feet: Option<ItemKind>,
}

impl EquipmentBuilder {
	fn new(weapon: ItemKind) -> Self {
		Self {
			weapon,
			head: None,
			chest: None,
			legs: None,
			feet: None,
		}
	}
	fn head(mut self, head: ItemKind) -> Self {
		self.head = Some(head);
		self
	}
	fn chest(mut self, chest: ItemKind) -> Self {
		self.chest = Some(chest);
		self
	}
	fn legs(mut self, legs: ItemKind) -> Self {
		self.legs = Some(legs);
		self
	}
	fn feet(mut self, feet: ItemKind) -> Self {
		self.feet = Some(feet);
		self
	}
	fn build(self) -> Equipment {
		Equipment {
			weapon: self.weapon,
			head: self.head,
			chest: self.chest,
			legs: self.legs,
			feet: self.feet,
		}
	}
}

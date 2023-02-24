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
		set_slot(client, 0, Some(equipment.weapon));
		set_slot(client, 38, equipment.chest);
		set_slot(client, 37, equipment.legs);
		set_slot(client, 39, equipment.head);
		set_slot(client, 36, equipment.feet);
	}
}

fn set_slot(client: &mut Mut<Client>, id: i16, item: Option<ItemKind>) {
	let item_stack = item.and_then(|item| Some(ItemStack::new(item, 1, None)));
	client.write_packet(&SetContainerSlot {
		window_id: -2,
		state_id: VarInt(0),
		slot_idx: id,
		slot_data: item_stack,
	});
}

#[derive(Resource)]
pub struct EquipmentLevel {
	pub equipments: Vec<Equipment>,
}

impl EquipmentLevel {
	pub fn new() -> Self {
		let mut equipments: Vec<Equipment> = Vec::new();

		// wood / leather
		let mut equipment = Equipment::new(ItemKind::WoodenSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::LeatherChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::LeatherLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::LeatherHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::LeatherBoots);
		equipments.push(equipment.clone());

		// gold
		equipment.set_weapon(ItemKind::GoldenSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::GoldenChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::GoldenLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::GoldenHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::GoldenBoots);
		equipments.push(equipment.clone());

		// stone / chainmail
		equipment.set_weapon(ItemKind::StoneSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::ChainmailChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::ChainmailLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::ChainmailHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::ChainmailBoots);
		equipments.push(equipment.clone());

		// iron
		equipment.set_weapon(ItemKind::IronSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::IronChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::IronLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::IronHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::IronBoots);
		equipments.push(equipment.clone());

		// diamond
		equipment.set_weapon(ItemKind::DiamondSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::DiamondChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::DiamondLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::DiamondHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::DiamondBoots);
		equipments.push(equipment.clone());

		// netherite
		equipment.set_weapon(ItemKind::NetheriteSword);
		equipments.push(equipment.clone());
		equipment.set_chest(ItemKind::NetheriteChestplate);
		equipments.push(equipment.clone());
		equipment.set_legs(ItemKind::NetheriteLeggings);
		equipments.push(equipment.clone());
		equipment.set_head(ItemKind::NetheriteHelmet);
		equipments.push(equipment.clone());
		equipment.set_feet(ItemKind::NetheriteBoots);
		equipments.push(equipment);

		// trident / turtle
		let equipment = Equipment {
			weapon: ItemKind::Trident,
			head: Some(ItemKind::TurtleHelmet),
			chest: None,
			legs: None,
			feet: None,
		};
		equipments.push(equipment);

		Self { equipments }
	}
	fn get(&self, level: &Level) -> Option<&Equipment> {
		self.equipments.get(level.0 as usize)
	}
}

#[derive(Clone)]
pub struct Equipment {
	weapon: ItemKind,
	head: Option<ItemKind>,
	chest: Option<ItemKind>,
	legs: Option<ItemKind>,
	feet: Option<ItemKind>,
}

impl Equipment {
	fn new(weapon: ItemKind) -> Self {
		Self {
			weapon,
			head: None,
			chest: None,
			legs: None,
			feet: None,
		}
	}
	fn set_weapon(&mut self, weapon: ItemKind) {
		self.weapon = weapon;
	}
	fn set_head(&mut self, head: ItemKind) {
		self.head = Some(head);
	}
	fn set_chest(&mut self, chest: ItemKind) {
		self.chest = Some(chest);
	}
	fn set_legs(&mut self, legs: ItemKind) {
		self.legs = Some(legs);
	}
	fn set_feet(&mut self, feet: ItemKind) {
		self.feet = Some(feet);
	}
}

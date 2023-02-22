mod chat;
mod combat;
mod level;
mod server_list_ping;
mod world;

use level::EquipmentLevel;
use valence::client::despawn_disconnected_clients;
use valence::client::event::default_event_handler;
use valence::prelude::*;
use valence::protocol::packets::s2c::play::WorldBorderInitialize;
use valence::protocol::{VarInt, VarLong};

pub fn main() {
	App::new()
		.insert_resource(EquipmentLevel::new())
		.add_plugin(ServerPlugin::new(server_list_ping::MyCallbacks))
		.add_startup_system(world::setup)
		.add_system_to_stage(EventLoop, default_event_handler)
		.add_system_to_stage(EventLoop, combat::handle_combat_events)
		.add_system_to_stage(EventLoop, chat::handle_message_events)
		.add_system(init_clients)
		.add_system(despawn_disconnected_clients)
		.add_system_set(PlayerList::default_system_set())
		.add_system(teleport_oob_clients)
		.run();
}

fn init_clients(
	mut commands: Commands,
	mut clients: Query<(Entity, &mut Client), Added<Client>>,
	instances: Query<Entity, With<Instance>>,
) {
	let instance = instances.single();

	for (entity, mut client) in &mut clients {
		client.set_game_mode(GameMode::Adventure);
		client.set_position([0.0, 3.0, 0.0]);
		client.set_instance(instance);
		set_world_border_size(&mut client, 30.0);

		commands.entity(entity).insert((
			combat::CombatState::default(),
			level::Level::default(),
			McEntity::with_uuid(EntityKind::Player, instance, client.uuid()),
		));
	}
}

fn set_world_border_size(client: &mut Mut<Client>, diameter: f64) {
	client.write_packet(&WorldBorderInitialize {
		x: 0.0,
		z: 0.0,
		old_diameter: diameter,
		new_diameter: diameter,
		speed: VarLong(0),
		portal_teleport_boundary: VarInt(29999984),
		warning_blocks: VarInt(0),
		warning_time: VarInt(0),
	});
}

fn teleport_oob_clients(mut clients: Query<&mut Client>) {
	for mut client in &mut clients {
		if client.position().y < -15.0 {
			client.set_position([0.0, 3.0, 0.0]);
			client.player_mut().set_health(20.0);
		}
	}
}

mod combat;
mod commands;
mod server_list_ping;

use valence::client::despawn_disconnected_clients;
use valence::client::event::default_event_handler;
use valence::prelude::*;
use valence::protocol::packets::s2c::play::WorldBorderInitialize;
use valence::protocol::{VarInt, VarLong};
use valence_anvil::AnvilWorld;

pub fn main() {
	App::new()
		.add_plugin(ServerPlugin::new(server_list_ping::MyCallbacks))
		.add_startup_system(setup)
		.add_system_to_stage(EventLoop, default_event_handler)
		.add_system_to_stage(EventLoop, combat::handle_combat_events)
		.add_system_to_stage(EventLoop, commands::interpret_command)
		.add_system(init_clients)
		.add_system(despawn_disconnected_clients)
		.add_system_set(PlayerList::default_system_set())
		.add_system(teleport_oob_clients)
		.run();
}

fn setup(world: &mut World) {
	let mut instance = world
		.resource::<Server>()
		.new_instance(DimensionId::default());

	let mut anvil = AnvilWorld::new("world");
	for x in -2..2 {
		for z in -2..2 {
			if let Ok(Some(anvil_chunk)) = anvil.read_chunk(x, z) {
				let mut chunk = Chunk::new(24);
				valence_anvil::to_valence(&anvil_chunk.data, &mut chunk, 4, |_| BiomeId::default())
					.unwrap();
				instance.insert_chunk([x, z], chunk);
			} else {
				instance.insert_chunk([x, z], Chunk::default());
			}
		}
	}

	world.spawn(instance);
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
		client.write_packet(&WorldBorderInitialize {
			x: 0.0,
			z: 0.0,
			old_diameter: 30.0,
			new_diameter: 30.0,
			speed: VarLong(0),
			portal_teleport_boundary: VarInt(29999984),
			warning_blocks: VarInt(0),
			warning_time: VarInt(0),
		});

		commands.entity(entity).insert((
			combat::CombatState {
				last_attacked_tick: 0,
				has_bonus_knockback: false,
				health: 20.0,
			},
			McEntity::with_uuid(EntityKind::Player, instance, client.uuid()),
		));
	}
}

fn teleport_oob_clients(mut clients: Query<&mut Client>) {
	for mut client in &mut clients {
		if client.position().y < -15.0 {
			client.set_position([0.0, 3.0, 0.0]);
			client.player_mut().set_health(20.0);
		}
	}
}

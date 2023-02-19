mod server_list_ping;

use valence::client::despawn_disconnected_clients;
use valence::client::event::{
	default_event_handler, ChatCommand, InteractWithEntity, StartSprinting, StopSprinting,
};
use valence::math::Vec3Swizzles;
use valence::prelude::*;
use valence::protocol::packets::s2c::play::WorldBorderInitialize;
use valence::protocol::{VarInt, VarLong};

#[derive(Component)]
struct CombatState {
	last_attacked_tick: i64,
	has_bonus_knockback: bool,
}

pub fn main() {
	App::new()
		.add_plugin(ServerPlugin::new(server_list_ping::MyCallbacks))
		.add_startup_system(setup)
		.add_system_to_stage(EventLoop, default_event_handler)
		.add_system_to_stage(EventLoop, handle_combat_events)
		.add_system_to_stage(EventLoop, interpret_command)
		.add_system(init_clients)
		.add_system(despawn_disconnected_clients)
		.add_system_set(PlayerList::default_system_set())
		.run();
}

fn setup(world: &mut World) {
	let mut instance = world
		.resource::<Server>()
		.new_instance(DimensionId::default());

	for z in -2..2 {
		for x in -2..2 {
			instance.insert_chunk([x, z], Chunk::default());
		}
	}

	for z in -15..15 {
		for x in -15..15 {
			instance.set_block_state([x, 0, z], BlockState::DEEPSLATE_TILES);
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
		client.set_position([0.0, 1.0, 0.0]);
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
			CombatState {
				last_attacked_tick: 0,
				has_bonus_knockback: false,
			},
			McEntity::with_uuid(EntityKind::Player, instance, client.uuid()),
		));
	}
}

fn handle_combat_events(
	manager: Res<McEntityManager>,
	server: Res<Server>,
	mut start_sprinting: EventReader<StartSprinting>,
	mut stop_sprinting: EventReader<StopSprinting>,
	mut interact_with_entity: EventReader<InteractWithEntity>,
	mut clients: Query<(&mut Client, &mut CombatState, &mut McEntity)>,
) {
	for &StartSprinting { client } in start_sprinting.iter() {
		if let Ok((_, mut state, _)) = clients.get_mut(client) {
			state.has_bonus_knockback = true;
		}
	}

	for &StopSprinting { client } in stop_sprinting.iter() {
		if let Ok((_, mut state, _)) = clients.get_mut(client) {
			state.has_bonus_knockback = false;
		}
	}

	for &InteractWithEntity {
		client: attacker_client,
		entity_id,
		..
	} in interact_with_entity.iter()
	{
		let Some(victim_client) = manager.get_with_protocol_id(entity_id) else {
			continue
		};

		let Ok([(attacker_client, mut attacker_state, _), (mut victim_client, mut victim_state, mut victim_entity)]) =
			clients.get_many_mut([attacker_client, victim_client])
		else {
			continue
		};

		if server.current_tick() - victim_state.last_attacked_tick < 10 {
			continue;
		}

		victim_state.last_attacked_tick = server.current_tick();

		let victim_pos = victim_client.position().xz();
		let attacker_pos = attacker_client.position().xz();

		let dir = (victim_pos - attacker_pos).normalize().as_vec2();

		let knockback_xz = if attacker_state.has_bonus_knockback {
			18.0
		} else {
			8.0
		};
		let knockback_y = if attacker_state.has_bonus_knockback {
			8.432
		} else {
			6.432
		};

		victim_client.set_velocity([dir.x * knockback_xz, knockback_y, dir.y * knockback_xz]);

		attacker_state.has_bonus_knockback = false;

		victim_client.trigger_status(EntityStatus::DamageFromGenericSource);
		victim_entity.trigger_status(EntityStatus::DamageFromGenericSource);
	}
}

fn interpret_command(mut clients: Query<&mut Client>, mut events: EventReader<ChatCommand>) {
	for event in events.iter() {
		let Ok(mut client) = clients.get_component_mut::<Client>(event.client) else {
			continue;
		};

		let mut args = event.command.split_whitespace();

		match args.next() {
			Some("help") => {
				client.send_message("commands:");
				client.send_message("- help");
				client.send_message("- spec");
			}
			Some("spec") => {
				let mode = match client.game_mode() {
					GameMode::Adventure => GameMode::Spectator,
					_ => GameMode::Adventure,
				};
				client.set_game_mode(mode);
			}
			_ => continue,
		}
	}
}

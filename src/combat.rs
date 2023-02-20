use valence::client::event::{InteractWithEntity, StartSprinting, StopSprinting};
use valence::math::Vec3Swizzles;
use valence::prelude::*;
use valence::protocol::types::SoundCategory;
use valence::protocol::Sound;

use crate::respawn;

#[derive(Component)]
pub struct CombatState {
	pub last_attacked_tick: i64,
	pub has_bonus_knockback: bool,
	pub health: f32,
}

pub fn handle_combat_events(
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

		let Ok([(mut attacker_client, mut attacker_state, _), (mut victim_client, mut victim_state, mut victim_entity)]) =
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

		victim_state.health -= 1.0;

		if victim_state.health < 0.5 {
			let pos = attacker_client.position();
			attacker_client.play_sound(
				Sound::EntityPlayerLevelup,
				SoundCategory::Player,
				pos,
				1.0,
				1.0,
			);
			respawn(&mut victim_client);
		}

		victim_client.trigger_status(EntityStatus::DamageFromGenericSource);
		victim_client.player_mut().set_health(victim_state.health);
		victim_entity.trigger_status(EntityStatus::DamageFromGenericSource);
	}
}

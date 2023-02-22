use valence::client::event::{InteractWithEntity, StartSprinting, StopSprinting};
use valence::math::Vec3Swizzles;
use valence::prelude::*;
use valence::protocol::types::SoundCategory;
use valence::protocol::Sound;

use crate::level::{EquipmentLevel, Level};

#[derive(Component, Default)]
pub struct CombatState {
	pub last_attacked_tick: i64,
	pub has_bonus_knockback: bool,
}

pub fn handle_combat_events(
	manager: Res<McEntityManager>,
	server: Res<Server>,
	equipment: Res<EquipmentLevel>,
	mut start_sprinting: EventReader<StartSprinting>,
	mut stop_sprinting: EventReader<StopSprinting>,
	mut interact_with_entity: EventReader<InteractWithEntity>,
	mut clients: Query<(&mut Client, &mut CombatState, &mut McEntity, &mut Level)>,
) {
	for &StartSprinting { client } in start_sprinting.iter() {
		if let Ok((_, mut state, ..)) = clients.get_mut(client) {
			state.has_bonus_knockback = true;
		}
	}

	for &StopSprinting { client } in stop_sprinting.iter() {
		if let Ok((_, mut state, ..)) = clients.get_mut(client) {
			state.has_bonus_knockback = false;
		}
	}

	for &InteractWithEntity {
		client: attacker_entity,
		entity_id,
		..
	} in interact_with_entity.iter()
	{
		let Some(victim_entity) = manager.get_with_protocol_id(entity_id) else {
			continue
		};

		let Ok([
			(mut attacker_client, mut attacker_state, _, mut attacker_level),
			(mut victim_client, mut victim_state, mut victim_mc_entity, mut victim_level)
			]) = clients.get_many_mut([attacker_entity, victim_entity])
		else {
			continue
		};

		// hit delay
		if server.current_tick() - victim_state.last_attacked_tick < 10 {
			continue;
		}

		let victim_pos = victim_client.position();
		let attacker_pos = attacker_client.position();

		if is_in_safe_zone(victim_pos) || is_in_safe_zone(attacker_pos) {
			continue;
		}

		victim_state.last_attacked_tick = server.current_tick();

		let victim_xz = victim_pos.xz();
		let attacker_xz = attacker_pos.xz();

		let dir = (victim_xz - attacker_xz).normalize().as_vec2();

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

		let mut health = victim_client.player().get_health() - 1.0;

		if health < 0.5 {
			play_sound(&mut attacker_client, Sound::EntityPlayerLevelup);
			victim_client.set_position([0.0, 3.0, 0.0]);
			health = 20.0;
			attacker_level.increase(&mut attacker_client, &equipment);
			victim_level.decrease(&mut victim_client, &equipment);
		}
		play_sound(&mut attacker_client, Sound::EntityPlayerHurt);

		victim_client.trigger_status(EntityStatus::DamageFromGenericSource);
		victim_client.player_mut().set_health(health);
		victim_mc_entity.trigger_status(EntityStatus::DamageFromGenericSource);
	}
}

fn is_in_safe_zone(pos: DVec3) -> bool {
	pos.x < 3.0 && pos.y > 2.75 && pos.z < 3.0
}

fn play_sound(client: &mut Mut<Client>, sound: Sound) {
	let pos = client.position();
	client.play_sound(sound, SoundCategory::Player, pos, 1.0, 1.0);
}

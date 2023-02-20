use valence::client::event::ChatCommand;
use valence::prelude::*;

pub fn interpret_command(mut clients: Query<&mut Client>, mut events: EventReader<ChatCommand>) {
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

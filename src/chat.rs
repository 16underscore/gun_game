use valence::{client::event::ChatMessage, prelude::*};

pub fn handle_message_events(
	mut clients: Query<&mut Client>,
	mut messages: EventReader<ChatMessage>,
) {
	for message in messages.iter() {
		let Ok(client) = clients.get_component::<Client>(message.client) else {
			continue;
		};

		let message = message.message.to_string();

		let formatted = format!("<{}> {}", client.username(), message);

		clients.par_for_each_mut(16, |mut client| client.send_message(formatted.clone()));
	}
}

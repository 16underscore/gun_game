use std::net::SocketAddr;

use valence::prelude::*;

pub struct MyCallbacks;

#[async_trait]
impl AsyncCallbacks for MyCallbacks {
	async fn server_list_ping(&self, _: &SharedServer, _: SocketAddr, _: i32) -> ServerListPing {
		ServerListPing::Respond {
			online_players: -1,
			max_players: 3,
			player_sample: Vec::new(),
			description: "a gun game server".into_text(),
			favicon_png: include_bytes!("../assets/logo-64x64.png"),
		}
	}
}

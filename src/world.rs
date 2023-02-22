use valence::prelude::*;
use valence_anvil::AnvilWorld;

pub fn setup(world: &mut World) {
	let mut instance = world
		.resource::<Server>()
		.new_instance(DimensionId::default());

	let mut anvil = AnvilWorld::new("world");
	for x in -2..2 {
		for z in -2..2 {
			if let Some(chunk) = get_chunk(x, z, &mut anvil) {
				instance.insert_chunk([x, z], chunk);
			} else {
				instance.insert_chunk([x, z], Chunk::default());
			}
		}
	}
	world.spawn(instance);
}

fn get_chunk(x: i32, z: i32, anvil: &mut AnvilWorld) -> Option<Chunk> {
	let anvil_chunk = anvil.read_chunk(x, z).ok()??;
	let mut chunk = Chunk::new(24);
	valence_anvil::to_valence(&anvil_chunk.data, &mut chunk, 4, |_| BiomeId::default()).ok()?;
	Some(chunk)
}

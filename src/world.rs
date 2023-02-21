use valence::prelude::*;
use valence_anvil::AnvilWorld;

pub fn setup(world: &mut World) {
	let mut instance = world
		.resource::<Server>()
		.new_instance(DimensionId::default());

	let mut anvil = AnvilWorld::new("world");
	for x in -2..2 {
		for z in -2..2 {
			if let Ok(Some(anvil_chunk)) = anvil.read_chunk(x, z) {
				let mut chunk = Chunk::new(24);
				if let Err(_) =
					valence_anvil::to_valence(&anvil_chunk.data, &mut chunk, 4, |_| BiomeId::default())
				{
					instance.insert_chunk([x, z], Chunk::default());
				} else {
					instance.insert_chunk([x, z], chunk);
				}
			} else {
				instance.insert_chunk([x, z], Chunk::default());
			}
		}
	}
	world.spawn(instance);
}

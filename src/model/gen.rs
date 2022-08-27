use super::*;

impl Model {
    pub fn generate_arena(&mut self) {
        self.generate_blocks();
    }

    fn generate_blocks(&mut self) {
        let mut rng = global_rng();
        let config = &self.assets.config;
        for _ in 0..100 {
            let position = Position::random(&mut rng, config.arena_size);
            if self.blocks.iter().any(|block| {
                block.position.distance(&position, config.arena_size) < config.blocks_spacing
            }) {
                continue;
            }
            let size = vec2(
                rng.gen_range(config.block_min_size.x..=config.block_max_size.x),
                rng.gen_range(config.block_min_size.y..=config.block_max_size.y),
            );
            let block = Block {
                id: self.id_gen.next(),
                position,
                collider: Collider::Aabb { size },
            };
            self.blocks.insert(block);
        }
    }
}

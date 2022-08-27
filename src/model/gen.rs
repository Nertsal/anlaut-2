use super::*;

impl Model {
    pub fn restart(&mut self) {
        self.blocks.clear();
        self.humans.clear();
        self.projectiles.clear();
        for gun in &mut self.guns {
            gun.velocity = Vec2::ZERO;
            gun.attached_human = None;
            gun.next_reload = Time::ZERO;
            gun.ammo = 0;
        }
        self.state = GameState::InProgress;
        self.generate_arena();
    }

    pub fn generate_arena(&mut self) {
        self.generate_blocks();
        self.spawn_humans();
    }

    fn generate_blocks(&mut self) {
        let mut rng = global_rng();
        let config = &self.assets.config;
        for _ in 0..config.blocks_number {
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

    fn spawn_humans(&mut self) {
        let mut rng = global_rng();
        let config = &self.assets.config;

        let players = self.players.len();
        let humans = config.singleplayer_humans
            + config.multiplayer_humans_delta * players.saturating_sub(1);

        while self.humans.len() < humans {
            let position = Position::random(&mut rng, config.arena_size);
            if self.blocks.iter().any(|block| {
                block.position.distance(&position, config.arena_size) < config.blocks_spacing
            }) {
                continue;
            }
            let human = Human {
                id: self.id_gen.next(),
                is_alive: true,
                position,
                velocity: Rotation::new(global_rng().gen_range(-Coord::PI..=Coord::PI)).direction(),
                collider: Collider::Aabb {
                    size: vec2(2.0, 2.0).map(Coord::new),
                },
                holding_gun: None,
                knock_out_timer: None,
            };
            self.humans.insert(human);
        }
    }
}

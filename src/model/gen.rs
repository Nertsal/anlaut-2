use super::*;

impl Model {
    pub fn restart(&mut self) {
        let config = &self.assets.config;

        self.blocks.clear();
        self.humans.clear();
        self.projectiles.clear();
        self.guns.clear();
        self.state = GameState::InProgress {
            time_left: config.round_time,
        };

        self.generate_arena();
        self.respawn_players();
    }

    pub fn generate_arena(&mut self) {
        self.generate_blocks();
        self.spawn_humans();
    }

    fn respawn_players(&mut self) {
        let mut rng = global_rng();
        let mut new_guns = Collection::new();
        for player in &mut self.players {
            let gun_id = self.id_gen.next();
            let gun = Gun {
                id: gun_id,
                owner: Some(player.id),
                death: None,
                position: Position::random(&mut rng, self.assets.config.arena_size),
                rotation: Rotation::ZERO,
                velocity: Vec2::ZERO,
                collider: Collider::Aabb {
                    size: self.assets.config.gun_size,
                },
                attached_human: None,
                aiming_at_host: false,
                next_reload: Time::ZERO,
                ammo: 0,
                invert_next_bullet: false,
            };
            new_guns.insert(gun);
            player.state = PlayerState::Gun { gun_id };
            player.score = 0;
        }
        self.guns = new_guns;
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

        let powerups = vec![PowerUp::Inversion];
        let human_types = vec![
            (R32::ONE, HumanType::Carrier { holding_gun: None }),
            (config.human_pusher_chance, HumanType::Pusher),
        ];

        while self.humans.len() < humans {
            let position = Position::random(&mut rng, config.arena_size);
            if self.blocks.iter().any(|block| {
                block.position.distance(&position, config.arena_size) < config.blocks_spacing
            }) {
                continue;
            }
            let powerup = rng
                .gen_bool(config.human_powerup_chance.as_f32() as f64)
                .then(|| powerups.choose(&mut rng))
                .flatten()
                .cloned();
            let human_type = human_types
                .choose_weighted(&mut rng, |(weight, _)| weight.as_f32())
                .unwrap()
                .1
                .clone();
            let human = Human {
                id: self.id_gen.next(),
                death: None,
                position,
                velocity: Rotation::new(global_rng().gen_range(-Coord::PI..=Coord::PI)).direction(),
                collider: Collider::Aabb {
                    size: vec2(2.0, 2.0).map(Coord::new),
                },
                human_type,
                knock_out_timer: None,
                holding_powerup: powerup,
            };
            self.humans.insert(human);
        }
    }
}

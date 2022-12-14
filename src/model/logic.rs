mod collisions;
mod deaths;
mod guns;
mod humans;
mod movement;

use super::*;

pub struct Logic<'a> {
    pub delta_time: Time,
    pub model: &'a mut Model,
    pub events: &'a mut Vec<Event>,
}

impl Model {
    pub fn update(&mut self, delta_time: Time, events: &mut Vec<Event>) {
        let mut logic = Logic {
            delta_time,
            model: self,
            events,
        };
        logic.process();
    }
}

impl Logic<'_> {
    pub fn process(&mut self) {
        self.process_spawns();
        self.process_guns();
        self.process_humans();
        self.process_inversions();
        self.process_movement();
        self.process_collisions();
        self.process_deaths();
        self.check_state();
    }

    fn process_spawns(&mut self) {
        let mut rng = global_rng();
        let mut new_guns = Vec::new();
        for player in &mut self.model.players {
            if let PlayerState::Respawning { time_left } = &mut player.state {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    let gun_id = self.model.id_gen.next();
                    let gun = Gun {
                        id: gun_id,
                        owner: Some(player.id),
                        death: None,
                        position: Position::random(&mut rng, self.model.assets.config.arena_size),
                        rotation: Rotation::ZERO,
                        velocity: Vec2::ZERO,
                        collider: Collider::Aabb {
                            size: self.model.assets.config.gun_size,
                        },
                        attached_human: None,
                        aiming_at_host: false,
                        next_reload: Time::ZERO,
                        ammo: 0,
                        invert_next_bullet: false,
                    };
                    new_guns.push(gun);
                    player.state = PlayerState::Gun { gun_id };
                }
            }
        }
        self.model.guns.extend(new_guns);
    }

    fn process_inversions(&mut self) {
        let config = &self.model.assets.config;
        for inversion in &mut self.model.inversions {
            let speed = if inversion.lifetime > config.inversion_shrink_time {
                config.inversion_speed
            } else {
                -config.inversion_max_radius / config.inversion_shrink_time
            };
            inversion.radius = (inversion.radius + speed * self.delta_time)
                .clamp(Coord::ZERO, config.inversion_max_radius);
        }
    }

    fn check_state(&mut self) {
        match &mut self.model.state {
            GameState::InProgress { time_left } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO || self.model.humans.is_empty() {
                    // Time is up or all humans killed
                    // The game is finished
                    self.model.state = GameState::Finished {
                        time_left: self.model.assets.config.game_restart_delay,
                        stats: GameStats {
                            scores: self
                                .model
                                .players
                                .iter()
                                .filter(|player| !matches!(player.state, PlayerState::Spectator))
                                .map(|player| (player.id, player.score))
                                .collect(),
                        },
                    };
                }
            }
            GameState::Finished { time_left, .. } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    // Start the game again
                    self.model.restart();
                }
            }
        }
    }

    fn apply_powerup(&mut self, gun_id: Id, powerup: PowerUp) {
        let _config = &self.model.assets.config;

        if let Some(gun) = self.model.guns.get_mut(&gun_id) {
            match powerup {
                PowerUp::Inversion => {
                    gun.invert_next_bullet = true;
                }
            }
        }
    }
}

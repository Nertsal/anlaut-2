mod collisions;
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
                        is_alive: true,
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
                    };
                    new_guns.push(gun);
                    player.state = PlayerState::Gun { gun_id };
                }
            }
        }
        self.model.guns.extend(new_guns);
    }

    fn process_deaths(&mut self) {
        let config = &self.model.assets.config;

        // Check for human deaths
        self.model.humans.retain(|human| human.is_alive);
        // Check for gun deaths
        for gun in &self.model.guns {
            if gun.is_alive {
                continue;
            }
            if let Some(player) = self.model.players.iter_mut().find(
                |player| matches!(player.state, PlayerState::Gun { gun_id } if gun_id == gun.id),
            ) {
                // Respawn player's gun
                player.state = PlayerState::Respawning {
                    time_left: config.gun_respawn_time,
                };
            }
        }
        self.model.guns.retain(|gun| gun.is_alive);

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
            if projectile.lifetime <= Time::ZERO {
                self.events.push(Event::ProjectileCollide {
                    position: projectile.position,
                })
            }
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }

    fn check_state(&mut self) {
        match &mut self.model.state {
            GameState::InProgress => {
                if self.model.humans.is_empty() {
                    // The game is finished
                    self.model.state = GameState::Finished {
                        time_left: self.model.assets.config.game_restart_delay,
                    };
                }
            }
            GameState::Finished { time_left } => {
                *time_left -= self.delta_time;
                if *time_left <= Time::ZERO {
                    // Start the game again
                    self.model.restart();
                }
            }
        }
    }
}

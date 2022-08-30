use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        let config = &self.model.assets.config;

        // Check for inversions
        for inversion in &mut self.model.inversions {
            inversion.lifetime -= self.delta_time;
        }
        self.model
            .inversions
            .retain(|inversion| inversion.lifetime > Time::ZERO);

        // Check for human deaths
        for human in &mut self.model.humans {
            if let Some(info) = &human.death {
                if let Some(player) = info.killer.and_then(|id| self.model.players.get_mut(&id)) {
                    let score = config.human_kill_score;
                    player.score += score;
                    self.events.push(Event::ScoreCollect {
                        player: player.id,
                        position: human.position,
                        score,
                    });
                }
                if let HumanType::Carrier {
                    holding_gun: Some(id),
                } = &human.human_type
                {
                    if let Some(gun) = self.model.guns.get_mut(id) {
                        gun.attached_human = None;
                    }
                }
                if let Some(powerup) = human.holding_powerup.take() {
                    let projectile = Projectile {
                        id: self.model.id_gen.next(),
                        caster: None,
                        lifetime: None,
                        position: human.position,
                        velocity: Vec2::ZERO,
                        collider: Collider::Aabb {
                            size: config.powerup_size,
                        },
                        is_powerup: Some(powerup),
                        is_inverted: false,
                    };
                    self.model.projectiles.insert(projectile);
                }
            }
        }
        self.model.humans.retain(|human| human.death.is_none());

        // Check for gun deaths
        for gun in &self.model.guns {
            if let Some(info) = &gun.death {
                if let Some(player) = self.model.players.iter_mut().find(
                    |player| matches!(player.state, PlayerState::Gun { gun_id } if gun_id == gun.id),
                ) {
                    // Respawn player's gun
                    player.state = PlayerState::Respawning {
                        time_left: config.gun_respawn_time,
                    };
                }
                // Do not count suicides for score
                if gun.owner != info.killer {
                    if let Some(player) = info.killer.and_then(|id| self.model.players.get_mut(&id))
                    {
                        let score = config.gun_kill_score;
                        player.score += score;
                        self.events.push(Event::ScoreCollect {
                            player: player.id,
                            position: gun.position,
                            score,
                        });
                    }
                }
                if let Some(human) = gun
                    .attached_human
                    .and_then(|id| self.model.humans.get_mut(&id))
                {
                    if let HumanType::Carrier { holding_gun } = &mut human.human_type {
                        *holding_gun = None;
                    }
                }
            }
        }
        self.model.guns.retain(|gun| gun.death.is_none());

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.model.projectiles {
            if let Some(time) = &mut projectile.lifetime {
                *time -= self.delta_time
            };
            if projectile
                .lifetime
                .map(|time| time <= Time::ZERO)
                .unwrap_or(false)
            {
                self.events.push(Event::ProjectileCollide {
                    position: projectile.position,
                    velocity: projectile.velocity,
                    powerup: projectile.is_powerup.clone(),
                });
                if projectile.is_inverted {
                    // Spawn an inverted explosion
                    let inversion = Inversion {
                        id: self.model.id_gen.next(),
                        caster: projectile.caster,
                        lifetime: config.inversion_lifetime,
                        position: projectile.position,
                        radius: Coord::ZERO,
                    };
                    self.model.inversions.insert(inversion);
                }
            }
        }
        self.model.projectiles.retain(|projectile| {
            projectile
                .lifetime
                .map(|time| time > Time::ZERO)
                .unwrap_or(true)
        });
    }
}

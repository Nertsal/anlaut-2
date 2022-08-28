use super::*;

impl Logic<'_> {
    pub fn process_deaths(&mut self) {
        let config = &self.model.assets.config;

        // Check for human deaths
        for human in &self.model.humans {
            if let Some(info) = &human.death {
                if let Some(player) = info.killer.and_then(|id| self.model.players.get_mut(&id)) {
                    player.score += config.human_kill_score;
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
                if let Some(player) = info.killer.and_then(|id| self.model.players.get_mut(&id)) {
                    player.score += config.gun_kill_score;
                }
            }
        }
        self.model.guns.retain(|gun| gun.death.is_none());

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
            if projectile.lifetime <= Time::ZERO {
                self.events.push(Event::ProjectileCollide {
                    position: projectile.position,
                    velocity: projectile.velocity,
                })
            }
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}

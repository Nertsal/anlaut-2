use super::*;

impl Logic<'_> {
    pub fn process_collisions(&mut self) {
        // Inversions
        self.inversion_collisions();
        // Projectile-(human, gun) collisions
        self.projectile_collisions();
        self.gun_human_collisions();
        self.gun_gun_collisions();
        self.block_collisions();
    }

    fn inversion_collisions(&mut self) {
        let config = &self.model.assets.config;

        for inversion in &mut self.model.inversions {
            for (position, death) in itertools::chain![
                self.model
                    .humans
                    .iter_mut()
                    .map(|human| (human.position, &mut human.death)),
                self.model
                    .guns
                    .iter_mut()
                    .map(|gun| (gun.position, &mut gun.death)),
            ] {
                if position.distance(&inversion.position, config.arena_size)
                    <= config.inversion_kill_radius
                {
                    *death = Some(DeathInfo {
                        killer: inversion.caster,
                    });
                }
            }
        }
    }

    fn projectile_collisions(&mut self) {
        let config = &self.model.assets.config;

        let mut powerups = Vec::new();
        for projectile in &mut self.model.projectiles {
            if projectile.is_powerup.is_none() {
                for human in &mut self.model.humans {
                    if human.death.is_some() {
                        continue;
                    }
                    if projectile.collider.check(
                        &human.collider,
                        projectile
                            .position
                            .direction(&human.position, config.arena_size),
                    ) {
                        // Collision detected -> hill the human
                        human.death = Some(DeathInfo {
                            killer: projectile.caster,
                        });
                        projectile.lifetime = Some(Time::ZERO);
                        break;
                    }
                }
            }
            for gun in &mut self.model.guns {
                if projectile.collider.check(
                    &gun.collider,
                    projectile
                        .position
                        .direction(&gun.position, config.arena_size),
                ) {
                    // Collision detected
                    if let Some(powerup) = &projectile.is_powerup {
                        powerups.push((gun.id, powerup.clone()));
                    } else {
                        gun.death = Some(DeathInfo {
                            killer: projectile.caster,
                        });
                    }
                    projectile.lifetime = Some(Time::ZERO);
                    break;
                }
            }
        }
        for (gun_id, powerup) in powerups {
            self.apply_powerup(gun_id, powerup);
        }
    }

    fn gun_human_collisions(&mut self) {
        let config = &self.model.assets.config;

        let mut powerups = Vec::new();
        for gun in &mut self.model.guns {
            if let Some(human_id) = &gun.attached_human {
                // Check if human is still alive
                if !self
                    .model
                    .humans
                    .get(human_id)
                    .map(|human| human.death.is_none())
                    .unwrap_or(false)
                {
                    // Human is dead -> drop the weapon
                    let gun_id = self
                        .model
                        .humans
                        .get_mut(human_id)
                        .unwrap()
                        .holding_gun
                        .take();
                    assert_eq!(
                        gun_id,
                        Some(gun.id),
                        "human's holding_gun and gun's attached_human are conflicting"
                    );
                    gun.attached_human = None;
                }
                continue;
            }
            // Check for collisions
            for human in &mut self.model.humans {
                if human.death.is_some()
                    || human.holding_gun.is_some()
                    || human.knock_out_timer.is_some()
                {
                    continue;
                }
                if gun.collider.check(
                    &human.collider,
                    gun.position.direction(&human.position, config.arena_size),
                ) {
                    // Collision detected -> attach the gun to the human
                    human.holding_gun = Some(gun.id);
                    gun.attached_human = Some(human.id);
                    if let Some(powerup) = human.holding_powerup.take() {
                        // Take powerup
                        powerups.push((gun.id, powerup));
                    }
                    break;
                }
            }
        }
        for (gun, powerup) in powerups {
            self.apply_powerup(gun, powerup);
        }
    }

    fn gun_gun_collisions(&mut self) {
        let config = &self.model.assets.config;

        let ids: Vec<_> = self.model.guns.ids().copied().collect();
        for (i, id) in ids.iter().enumerate() {
            let mut gun = self.model.guns.remove(id).unwrap();
            for id in ids.iter().skip(i + 1) {
                let other = self.model.guns.get_mut(id).unwrap();

                if let Some(collision) = gun.collider.collision(
                    &other.collider,
                    gun.position.direction(&other.position, config.arena_size),
                ) {
                    // Shift position
                    let offset = collision.normal * collision.penetration / Coord::new(2.0);
                    other.position.shift(offset, config.arena_size);
                    gun.position.shift(-offset, config.arena_size);

                    // Apply force
                    std::mem::swap(&mut other.velocity, &mut gun.velocity);
                }
            }
            self.model.guns.insert(gun);
        }
    }

    fn block_collisions(&mut self) {
        let config = &self.model.assets.config;

        for projectile in &mut self.model.projectiles {
            if projectile
                .lifetime
                .map(|time| time <= Time::ZERO)
                .unwrap_or(false)
            {
                continue;
            }
            for block in &self.model.blocks {
                if let Some(_collision) = block.collider.collision(
                    &projectile.collider,
                    block
                        .position
                        .direction(&projectile.position, config.arena_size),
                ) {
                    // Kill the projectile
                    projectile.lifetime = Some(Time::ZERO);
                }
            }
        }
        for human in &mut self.model.humans {
            for block in &self.model.blocks {
                if let Some(collision) = block.collider.collision(
                    &human.collider,
                    block.position.direction(&human.position, config.arena_size),
                ) {
                    human
                        .position
                        .shift(collision.normal * collision.penetration, config.arena_size);
                    // Remove velocity in the collision direction
                    human.velocity -=
                        collision.normal * Vec2::dot(human.velocity, collision.normal);
                }
            }
        }
        for gun in &mut self.model.guns {
            if gun.attached_human.is_some() {
                continue;
            }
            for block in &self.model.blocks {
                if let Some(collision) = block.collider.collision(
                    &gun.collider,
                    block.position.direction(&gun.position, config.arena_size),
                ) {
                    gun.position
                        .shift(collision.normal * collision.penetration, config.arena_size);
                    // Bounce
                    // In case velocity is collinear with the normal, ignore bounce
                    gun.velocity -= collision.normal
                        * Vec2::dot(gun.velocity, collision.normal).min(Coord::ZERO)
                        * (Coord::ONE + config.gun_bounciness);
                }
            }
        }
    }
}

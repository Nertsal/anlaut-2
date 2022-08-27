use super::*;

impl Logic<'_> {
    pub fn process_collisions(&mut self) {
        let config = &self.model.assets.config;

        // Check for projectile-(human, gun) collisions
        for projectile in &mut self.model.projectiles {
            for human in &mut self.model.humans {
                if projectile.collider.check(
                    &human.collider,
                    projectile
                        .position
                        .direction(&human.position, config.arena_size),
                ) {
                    // Collision detected -> hill the human
                    human.is_alive = false;
                    projectile.lifetime = Time::ZERO;
                    break;
                }
            }
            for gun in &mut self.model.guns {
                if projectile.collider.check(
                    &gun.collider,
                    projectile
                        .position
                        .direction(&gun.position, config.arena_size),
                ) {
                    // Collision detected -> kill the gun
                    gun.is_alive = false;
                    projectile.lifetime = Time::ZERO;
                    break;
                }
            }
        }

        // Check for gun-human collisions
        for gun in &mut self.model.guns {
            if let Some(human_id) = &gun.attached_human {
                // Check if human is still alive
                if !self
                    .model
                    .humans
                    .get(human_id)
                    .map(|human| human.is_alive)
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
                if !human.is_alive || human.holding_gun.is_some() || human.knock_out_timer.is_some()
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
                    break;
                }
            }
        }

        // Check for block collisions
        for projectile in &mut self.model.projectiles {
            if projectile.lifetime <= Time::ZERO {
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
                    projectile.lifetime = Time::ZERO;
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

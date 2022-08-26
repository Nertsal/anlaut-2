use super::*;

impl Logic<'_> {
    pub fn process_collisions(&mut self) {
        // Check for projectile-human collisions
        for projectile in &mut self.model.projectiles {
            for human in &mut self.model.humans {
                if projectile
                    .collider
                    .check(&human.collider, human.position - projectile.position)
                {
                    // Collision detected -> hill the human
                    human.is_alive = false;
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
                if gun
                    .collider
                    .check(&human.collider, human.position - gun.position)
                {
                    // Collision detected -> attach the gun to the human
                    human.holding_gun = Some(gun.id);
                    gun.attached_human = Some(human.id);
                    break;
                }
            }
        }
    }
}

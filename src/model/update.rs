use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.process_movement(delta_time);
        self.process_collisions(delta_time);
        self.process_deaths(delta_time);
    }

    fn process_movement(&mut self, delta_time: Time) {
        // Move projectiles
        for projectile in &mut self.projectiles {
            projectile.position += projectile.velocity * delta_time;
        }
    }

    fn process_collisions(&mut self, _delta_time: Time) {
        // Check for collisions with projectiles
        for projectile in &mut self.projectiles {
            for human in &mut self.humans {
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
    }

    fn process_deaths(&mut self, delta_time: Time) {
        // Check for human deaths
        self.humans.retain(|human| human.is_alive);

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.projectiles {
            projectile.lifetime -= delta_time;
        }
        self.projectiles
            .retain(|projectile| projectile.lifetime > Hp::ZERO);
    }
}

mod collisions;
mod humans;
mod movement;

use super::*;

pub struct Logic<'a> {
    pub delta_time: Time,
    pub model: &'a mut Model,
}

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        let mut logic = Logic {
            delta_time,
            model: self,
        };
        logic.process();
    }
}

impl Logic<'_> {
    pub fn process(&mut self) {
        self.process_humans();
        self.process_movement();
        self.process_collisions();
        self.process_deaths();
    }

    fn process_deaths(&mut self) {
        // Check for human deaths
        self.model.humans.retain(|human| human.is_alive);

        // Check for projectiles "deaths" (collisions or lifetime)
        for projectile in &mut self.model.projectiles {
            projectile.lifetime -= self.delta_time;
        }
        self.model
            .projectiles
            .retain(|projectile| projectile.lifetime > Time::ZERO);
    }
}

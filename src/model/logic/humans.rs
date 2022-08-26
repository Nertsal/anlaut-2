use super::*;

impl Logic<'_> {
    pub fn process_humans(&mut self) {
        let mut rng = global_rng();
        for human in &mut self.model.humans {
            // Knock out timer
            if let Some(timer) = &mut human.knock_out_timer {
                *timer -= self.delta_time;
                if *timer <= Time::ZERO {
                    human.knock_out_timer = None;
                }
            }

            // Behaviour
            if human.holding_gun.is_some() {
                // Run around (panic)
                let speed = Coord::new(HUMAN_RUN_SPEED);
                let angle_delta = Rotation::new(
                    r32(rng.gen_range(-HUMAN_TURN_SPEED..=HUMAN_TURN_SPEED)) * self.delta_time,
                );
                let rotation = Rotation::new(human.velocity.arg()) + angle_delta;
                human.velocity = rotation.direction() * speed;
            } else {
                // Walk around
                let speed = Coord::new(HUMAN_WALK_SPEED);
                let angle_delta = Rotation::new(
                    r32(rng.gen_range(-HUMAN_TURN_SPEED..=HUMAN_TURN_SPEED)) * self.delta_time,
                );
                let rotation = Rotation::new(human.velocity.arg()) + angle_delta;
                human.velocity = rotation.direction() * speed;
            }
        }
    }
}

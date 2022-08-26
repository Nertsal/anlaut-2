use super::*;

impl Logic<'_> {
    pub fn process_humans(&mut self) {
        for human in &mut self.model.humans {
            // Knock out timer
            if let Some(timer) = &mut human.knock_out_timer {
                *timer -= self.delta_time;
                if *timer <= Time::ZERO {
                    human.knock_out_timer = None;
                }
            }
        }
    }
}

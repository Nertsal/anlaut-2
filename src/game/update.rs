use super::*;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        self.model.send(Message::Aim { target: mouse_pos });
    }
}

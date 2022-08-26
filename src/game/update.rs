use super::*;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        let model = self.model.get();
        if let Some(player) = model.players.get(&self.player_id) {
            if let PlayerState::Gun { gun_id } = &player.state {
                if let Some(gun) = model.guns.get(gun_id) {
                    self.camera.center = gun.position;
                }
            }
        }

        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        self.model.send(Message::Aim { target: mouse_pos });
    }
}

use super::*;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        let model = self.model.get();
        let gun_position = model
            .players
            .get(&self.player_id)
            .and_then(|player| {
                if let PlayerState::Gun { gun_id } = &player.state {
                    Some(gun_id)
                } else {
                    None
                }
            })
            .and_then(|gun_id| model.guns.get(gun_id))
            .map(|gun| gun.position);

        if let Some(gun_pos) = gun_position {
            self.model.send(Message::Aim {
                rotation: (mouse_pos - gun_pos).arg(),
            })
        }
    }
}

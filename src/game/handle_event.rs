use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
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

        match event {
            geng::Event::MouseDown {
                button: geng::MouseButton::Left,
                ..
            } => {
                if let Some(gun_pos) = gun_position {
                    let direction = mouse_pos - gun_pos;
                    self.model.send(Message::Shoot { direction });
                }
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num2 => {
                    self.model.send(Message::SpawnHuman {
                        position: mouse_pos,
                    });
                }
                geng::Key::Num3 => {
                    self.model.send(Message::SpawnGun {
                        position: mouse_pos,
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}

use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let mouse_pos = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), mouse_pos)
            .map(Coord::new);

        match event {
            geng::Event::MouseDown { button, .. } => {
                let release = match button {
                    geng::MouseButton::Left => false,
                    geng::MouseButton::Right => true,
                    geng::MouseButton::Middle => return,
                };
                self.model.send(Message::Shoot { release });
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num2 => {
                    self.model.send(Message::SpawnHuman {
                        position: mouse_pos,
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}

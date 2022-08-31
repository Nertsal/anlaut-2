use geng::Draw2d;

use super::*;

pub struct MainMenu {
    geng: Geng,
    assets: Rc<Assets>,
    opt: Opt,
    transition: Option<Transition>,
    singleplayer_button: AABB<f32>,
    multiplayer_button: AABB<f32>,
}

enum Transition {
    Singleplayer,
    Multiplayer,
}

impl MainMenu {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, opt: Opt) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            opt,
            transition: None,
            singleplayer_button: AABB::ZERO,
            multiplayer_button: AABB::ZERO,
        }
    }

    fn click(&mut self, position: Vec2<f32>) {
        if self.singleplayer_button.contains(position) {
            self.transition = Some(Transition::Singleplayer);
        } else if self.multiplayer_button.contains(position) {
            self.transition = Some(Transition::Multiplayer);
        }
    }
}

impl geng::State for MainMenu {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let framebuffer_size = framebuffer.size().map(|x| x as f32);

        let singleplayer_button = AABB::point(vec2(0.4, 0.5))
            .extend_left(0.2)
            .extend_symmetric(vec2(0.0, 0.1));
        let multiplayer_button =
            singleplayer_button.translate(vec2((0.5 - singleplayer_button.center().x) * 2.0, 0.0));
        let layout = |aabb: AABB<f32>| {
            let size_ref = framebuffer_size.x.min(framebuffer_size.y);
            AABB::point(aabb.center() * framebuffer_size)
                .extend_symmetric(aabb.size() * size_ref / 2.0)
        };

        self.singleplayer_button = layout(singleplayer_button);
        self.multiplayer_button = layout(multiplayer_button);

        draw_2d::Quad::new(self.singleplayer_button, Rgba::WHITE).draw_2d(
            &self.geng,
            framebuffer,
            &geng::PixelPerfectCamera,
        );
        draw_2d::Quad::new(self.multiplayer_button, Rgba::WHITE).draw_2d(
            &self.geng,
            framebuffer,
            &geng::PixelPerfectCamera,
        );

        draw_2d::Text::unit(&**self.geng.default_font(), "Singleplayer", Rgba::BLACK)
            .fit_into(self.singleplayer_button)
            .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
        draw_2d::Text::unit(&**self.geng.default_font(), "Multiplayer", Rgba::BLACK)
            .fit_into(self.multiplayer_button)
            .draw_2d(&self.geng, framebuffer, &geng::PixelPerfectCamera);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                self.click(position.map(|x| x as f32));
            }
            geng::Event::TouchStart { touches } => {
                if let [point] = touches[..] {
                    self.click(point.position.map(|x| x as f32));
                }
            }
            _ => {}
        }
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        let transition = self.transition.take()?;
        let state: Box<dyn geng::State> = match transition {
            Transition::Singleplayer => {
                let mut model = crate::model::Model::new(self.assets.server.clone());
                let player_id = model.new_player();
                let model = Connection::local(model, player_id);
                Box::new(game::Game::new(&self.geng, &self.assets, player_id, model))
            }
            Transition::Multiplayer => Box::new(simple_net::ConnectingState::new(
                &self.geng,
                self.opt.connect.as_deref().unwrap(),
                {
                    let geng = self.geng.clone();
                    let assets = self.assets.clone();
                    move |player_id, model| {
                        game::Game::new(&geng, &assets, player_id, Connection::Remote(model))
                    }
                },
            )),
        };
        Some(geng::Transition::Push(state))
    }
}

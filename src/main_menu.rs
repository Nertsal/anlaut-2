use geng::Draw2d;

use super::*;

use crate::model::*;

pub struct MainMenu {
    geng: Geng,
    assets: Rc<Assets>,
    opt: Opt,
    game_time: Time,
    camera: geng::Camera2d,
    framebuffer_size: Vec2<usize>,
    transition: Option<Transition>,
    explosion: Option<(Vec2<Coord>, Coord, Transition)>,
    world_size: Vec2<Coord>,
    humans: Vec<(Vec2<Coord>, Collider, Transition)>,
    projectile: Option<(Vec2<Coord>, Vec2<Coord>, Collider)>,
    gun: (Vec2<Coord>, Rotation, Collider),
    temp_texture: ugli::Texture,
}

#[derive(Debug, Clone, Copy)]
enum Transition {
    Singleplayer,
    Multiplayer,
}

impl MainMenu {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, opt: Opt) -> Self {
        let world_size = vec2(40.0, 40.0).map(Coord::new);
        let human_collider = Collider::Aabb {
            size: vec2(2.0, 2.0).map(Coord::new),
        };
        let left_pos = vec2(-10.0, 0.0).map(Coord::new);
        let gun_collider = Collider::Aabb {
            size: assets.server.config.gun_size,
        };
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            opt,
            framebuffer_size: vec2(1, 1),
            camera: geng::Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 20.0,
            },
            game_time: Time::ZERO,
            transition: None,
            explosion: None,
            world_size,
            humans: vec![
                (left_pos, human_collider.clone(), Transition::Singleplayer),
                (-left_pos, human_collider, Transition::Multiplayer),
            ],
            projectile: None,
            gun: (Vec2::ZERO, Rotation::ZERO, gun_collider),
            temp_texture: ugli::Texture::new_uninitialized(geng.ugli(), vec2(1, 1)),
        }
    }

    fn click(&mut self, position: Vec2<f32>) {
        let position = self
            .camera
            .screen_to_world(self.framebuffer_size.map(|x| x as f32), position)
            .map(Coord::new);
        if self.projectile.is_none() && self.explosion.is_none() {
            let velocity = (position - self.gun.0).normalize_or_zero()
                * self.assets.server.config.gun_shoot_speed;
            let collider = Collider::Aabb {
                size: vec2(0.2, 0.2).map(Coord::new),
            };
            self.projectile = Some((Vec2::ZERO, velocity, collider));
        }
    }
}

impl geng::State for MainMenu {
    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as f32);
        self.game_time += delta_time;
        if let Some((position, velocity, collider)) = &mut self.projectile {
            *position += *velocity * delta_time;
            let mut kill = position.x.abs() > self.world_size.x / Coord::new(2.0)
                || position.y.abs() > self.world_size.y / Coord::new(2.0);
            for (pos, col, transition) in &self.humans {
                if collider.check(col, *pos - *position) {
                    // Initiate an explosion
                    self.explosion = Some((*position, Coord::ZERO, *transition));
                    kill = true;
                }
            }
            if kill {
                self.projectile.take();
            }
        }
        if let Some((position, radius, transition)) = &mut self.explosion {
            *radius += Coord::new(50.0) * delta_time;
            let delta = position.map(|x| x.abs()) + self.world_size;
            if delta.len() * Coord::new(1.2) < *radius {
                // The explosion has covered the whole screen
                self.transition = Some(*transition);
            }
        }
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        if self.framebuffer_size != framebuffer.size() {
            self.temp_texture =
                ugli::Texture::new_uninitialized(self.geng.ugli(), framebuffer.size());
        }
        self.framebuffer_size = framebuffer.size();
        let framebuffer_size = framebuffer.size().map(|x| x as f32);

        use crate::game::render;

        render::field::draw_field(
            self.game_time,
            &self.assets.shaders.field,
            &self.geng,
            framebuffer,
            &self.camera,
        );

        for (position, collider, _) in &self.humans {
            let transform = Mat3::translate(*position);
            render::util::draw_collider(
                collider,
                transform,
                self.assets.colors.human_pusher,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        {
            let (position, rotation, collider) = &self.gun;
            let transform = Mat3::translate(*position) * Mat3::rotate(rotation.angle());
            render::util::draw_collider(
                collider,
                transform,
                Rgba::BLUE,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for (position, _, collider) in &self.projectile {
            let transform = Mat3::translate(*position);
            render::util::draw_collider(
                collider,
                transform,
                self.assets.colors.bullet,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }

        framebuffer.copy_to_texture(
            &mut self.temp_texture,
            AABB::ZERO.extend_positive(framebuffer.size()),
            Vec2::ZERO,
        );
        let (position, radius) = self
            .explosion
            .map(|(pos, radius, _)| (pos, radius))
            .or_else(|| {
                self.projectile
                    .as_ref()
                    .map(|(pos, _, _)| (*pos, Coord::new(0.5)))
            })
            .unwrap_or_else(|| (self.gun.0, Coord::new(0.5)));
        {
            let transform =
                Mat3::translate(position.map(Coord::as_f32)) * Mat3::scale_uniform(radius.as_f32());
            ugli::draw(
                framebuffer,
                &self.assets.shaders.inverted_explosion,
                ugli::DrawMode::TriangleFan,
                &render::unit_quad(self.geng.ugli()),
                (
                    ugli::uniforms! {
                        u_time: self.game_time.as_f32(),
                        u_model_matrix: transform,
                        u_frame_texture: &self.temp_texture,
                        u_frame_texture_size: self.temp_texture.size(),
                    },
                    geng::camera2d_uniforms(&self.camera, framebuffer_size),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        }
        for &(position, radius, _) in &self.explosion {
            draw_2d::Ellipse::circle(
                position.map(Coord::as_f32),
                radius.as_f32() / 2.0,
                Rgba::WHITE,
            )
            .draw_2d(&self.geng, framebuffer, &self.camera);
        }

        for (position, _, transition) in &self.humans {
            let text = match transition {
                Transition::Singleplayer => "Singleplayer",
                Transition::Multiplayer => "Multiplayer",
            };
            draw_2d::Text::unit(&**self.geng.default_font(), text, Rgba::WHITE)
                .scale_uniform(0.5)
                .translate(position.map(Coord::as_f32) + vec2(0.0, 5.0))
                .draw_2d(&self.geng, framebuffer, &self.camera);
        }
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                self.click(position.map(|x| x as f32));
            }
            geng::Event::MouseMove { position, .. } => {
                let position = self
                    .camera
                    .screen_to_world(
                        self.framebuffer_size.map(|x| x as f32),
                        position.map(|x| x as f32),
                    )
                    .map(Coord::new);
                let direction = position - self.gun.0;
                self.gun.1 = Rotation::new(direction.arg());
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
            Transition::Multiplayer => {
                let connect = self.opt.connect.as_deref()?;
                Box::new(simple_net::ConnectingState::new(&self.geng, connect, {
                    let geng = self.geng.clone();
                    let assets = self.assets.clone();
                    move |player_id, model| {
                        game::Game::new(&geng, &assets, player_id, Connection::Remote(model))
                    }
                }))
            }
        };
        Some(geng::Transition::Push(state))
    }
}

use super::*;

impl Render {
    fn get_position(&self, id: Id, position: Position) -> Position {
        self.interpolated_positions
            .get(&id)
            .map(|interpolation| interpolation.get_interpolated())
            .unwrap_or(position)
    }

    pub fn draw_world(
        &self,
        game_time: Time,
        model: &Model,
        framebuffer: &mut ugli::Framebuffer,
        temp_texture: &mut ugli::Texture,
    ) {
        let config = &model.assets.config;

        // Background field
        self.draw_field(game_time, framebuffer);

        let framebuffer_size = framebuffer.size().map(|x| Coord::new(x as f32));
        let camera_view = AABB::point(self.camera.center.to_world()).extend_symmetric(
            vec2(
                self.camera.fov * framebuffer_size.x / framebuffer_size.y,
                self.camera.fov,
            ) / Coord::new(2.0),
        );
        let camera_collider = Collider::Aabb {
            size: camera_view.size(),
        };

        let mut inversions: Vec<_> = model
            .inversions
            .iter()
            .map(|inversion| (inversion.position, inversion.radius))
            .collect();

        for human in &model.humans {
            let delta = self
                .camera
                .center
                .direction(&human.position, config.arena_size);
            let draw = |transform: Mat3<Coord>, alpha: f32, framebuffer: &mut ugli::Framebuffer| {
                match &human.human_type {
                    HumanType::Carrier { .. } => {
                        let mut color = Rgba::GREEN;
                        color.a = alpha;
                        draw_collider(
                            &human.collider,
                            transform,
                            color,
                            &self.geng,
                            framebuffer,
                            &self.camera,
                        );
                    }
                    HumanType::Pusher => {
                        let mut color = Rgba::YELLOW;
                        color.a = alpha;
                        draw_triangle_frame(
                            transform,
                            Coord::new(0.2),
                            color,
                            &self.geng,
                            framebuffer,
                            &self.camera,
                        );
                    }
                }
            };
            if camera_collider.check(&human.collider, delta) {
                // Human is in view
                let position = self.get_position(human.id, human.position);
                if let Some(powerup) = &human.holding_powerup {
                    self.draw_powerup(
                        powerup,
                        position,
                        model,
                        &self.geng,
                        framebuffer,
                        &self.camera,
                    );
                    inversions.push((position, Coord::new(0.5)));
                }
                let transform =
                    get_transform(position, Rotation::ZERO, config.arena_size, &self.camera);
                draw(transform, 1.0, framebuffer);
            } else {
                // Outside of view -> draw shadow
                let offset = (camera_view.center() + delta).clamp_aabb(camera_view);
                let transform = Mat3::translate(offset);
                draw(transform, 0.2, framebuffer);
            }
        }
        for gun in &model.guns {
            let position = self.get_position(gun.id, gun.position);
            if gun.invert_next_bullet {
                inversions.push((position, Coord::new(0.5)));
            }
            self.draw_gun(gun, model, &self.geng, framebuffer, &self.camera);
        }
        for projectile in &model.projectiles {
            let color = powerup_color(projectile.is_powerup.as_ref());
            let position = self.get_position(projectile.id, projectile.position);
            if projectile.is_powerup.is_some() || projectile.is_inverted {
                inversions.push((projectile.position, Coord::new(0.5)));
            }
            draw_collider(
                &projectile.collider,
                get_transform(position, Rotation::ZERO, config.arena_size, &self.camera),
                color,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for block in &model.blocks {
            draw_collider(
                &block.collider,
                get_transform(
                    self.get_position(block.id, block.position),
                    Rotation::ZERO,
                    config.arena_size,
                    &self.camera,
                ),
                Rgba::GRAY,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for particle in &self.particles {
            let scale = particle.lifetime.min(Time::new(0.2)) / Time::new(0.2);
            draw_quad_frame(
                AABB::ZERO.extend_symmetric(particle.size * scale / Coord::new(2.0)),
                get_transform(
                    particle.position,
                    Rotation::ZERO,
                    config.arena_size,
                    &self.camera,
                ),
                Coord::new(0.05),
                particle.color,
                &self.geng,
                framebuffer,
                &self.camera,
            )
        }
        for text in &self.texts {
            let position = self.camera.project_f32(text.position, config.arena_size);
            let font_size = text.size * text.lifetime.min(Time::new(1.5));
            let font = &**self.geng.default_font();
            draw_2d::Text::unit(font, &text.text, text.color)
                .scale_uniform(font_size.as_f32())
                .translate(position)
                .draw_2d(&self.geng, framebuffer, &self.camera);
        }

        // Inverted shader
        framebuffer.copy_to_texture(
            temp_texture,
            AABB::ZERO.extend_positive(framebuffer.size()),
            Vec2::ZERO,
        );
        for (position, radius) in inversions {
            let position = self.camera.project_f32(position, config.arena_size);
            let transform = Mat3::translate(position) * Mat3::scale_uniform(radius.as_f32());
            ugli::draw(
                framebuffer,
                &*self.assets.shaders.inverted_explosion,
                ugli::DrawMode::TriangleFan,
                &unit_quad(self.geng.ugli()),
                (
                    ugli::uniforms! {
                        u_time: game_time.as_f32(),
                        u_model_matrix: transform,
                        u_world_size: config.arena_size.map(Coord::as_f32),
                        u_frame_texture: &*temp_texture,
                        u_frame_texture_size: temp_texture.size(),
                    },
                    geng::camera2d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        }
    }

    fn draw_powerup(
        &self,
        powerup: &PowerUp,
        position: Position,
        model: &Model,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &CameraTorus2d,
    ) {
        let config = &model.assets.config;
        let position = camera.project(position, config.arena_size);
        match powerup {
            PowerUp::Inversion => {
                draw_quad_frame(
                    AABB::ZERO.extend_symmetric(config.powerup_size / Coord::new(2.0)),
                    Mat3::translate(position),
                    Coord::new(0.05),
                    Rgba::new(0.0, 0.5, 0.9, 0.7),
                    geng,
                    framebuffer,
                    camera,
                );
            }
        }
    }

    fn draw_gun(
        &self,
        gun: &Gun,
        model: &Model,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &CameraTorus2d,
    ) {
        let config = &model.assets.config;
        let gun_transform = get_transform(
            self.get_position(gun.id, gun.position),
            gun.rotation,
            config.arena_size,
            camera,
        );
        draw_collider(
            &gun.collider,
            gun_transform,
            Rgba::BLUE,
            geng,
            framebuffer,
            camera,
        );

        // Ammo
        let mut draw_ammo = |i: usize, color: Rgba<f32>| {
            let (right, width, height) = match gun.collider {
                Collider::Aabb { size } => {
                    let size = size * Coord::new(0.8);
                    (size.x / Coord::new(2.0), size.x, size.y)
                }
            };
            let bullet_spacing = width / Coord::new(10.0);
            let bullet_width = width / Coord::new((config.gun_magazine_size - 1) as f32);
            let bullet_aabb = AABB::ZERO
                .extend_symmetric(vec2(bullet_width - bullet_spacing, height) / Coord::new(2.0));
            let offset = right - Coord::new(i as f32) * bullet_width;
            let transform = gun_transform * Mat3::translate(vec2(offset, Coord::ZERO));
            draw_quad_frame(
                bullet_aabb,
                transform,
                Coord::new(0.05),
                color,
                geng,
                framebuffer,
                camera,
            );
        };
        for i in 0..gun.ammo {
            draw_ammo(i, Rgba::RED);
        }
    }
}

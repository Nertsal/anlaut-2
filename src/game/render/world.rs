use super::*;

impl Game {
    fn get_position(&self, id: Id, position: Position) -> Position {
        self.interpolated_positions
            .get(&id)
            .map(|interpolation| interpolation.get_interpolated())
            .unwrap_or(position)
    }

    pub fn draw_world(&self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();
        let config = &model.assets.config;

        // Background field
        self.draw_field(framebuffer);

        for human in &model.humans {
            draw_collider(
                &human.collider,
                get_transform(
                    self.get_position(human.id, human.position),
                    Rotation::ZERO,
                    config.arena_size,
                    &self.camera,
                ),
                Rgba::GREEN,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for gun in &model.guns {
            self.draw_gun(gun, &*model, &self.geng, framebuffer, &self.camera);
        }
        for projectile in &model.projectiles {
            draw_collider(
                &projectile.collider,
                get_transform(
                    self.get_position(projectile.id, projectile.position),
                    Rotation::ZERO,
                    config.arena_size,
                    &self.camera,
                ),
                Rgba::RED,
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

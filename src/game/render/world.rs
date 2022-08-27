use super::*;

impl Game {
    pub fn draw_world(&self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();
        let config = &model.assets.config;

        // Background field
        self.draw_field(framebuffer);

        for human in &model.humans {
            draw_collider(
                &human.collider,
                get_transform(
                    human.position,
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
            draw_collider(
                &gun.collider,
                get_transform(gun.position, gun.rotation, config.arena_size, &self.camera),
                Rgba::BLUE,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for projectile in &model.projectiles {
            draw_collider(
                &projectile.collider,
                get_transform(
                    projectile.position,
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
                    block.position,
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
}

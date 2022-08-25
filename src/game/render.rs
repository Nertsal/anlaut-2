use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let model = self.model.get();
        for human in &model.humans {
            draw_collider(
                human.position,
                &human.collider,
                Rgba::GREEN,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for gun in &model.guns {
            draw_collider(
                gun.position,
                &gun.collider,
                Rgba::BLUE,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
        for projectile in &model.projectiles {
            draw_collider(
                projectile.position,
                &projectile.collider,
                Rgba::RED,
                &self.geng,
                framebuffer,
                &self.camera,
            );
        }
    }
}

pub fn draw_collider(
    position: Position,
    collider: &Collider,
    color: Rgba<f32>,
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    match collider {
        &Collider::Aabb { size } => {
            let aabb = AABB::point(position)
                .extend_symmetric(size / Coord::new(2.0))
                .map(|x| x.as_f32());
            draw_2d::Quad::new(aabb, color).draw_2d(geng, framebuffer, camera);
        }
    }
}

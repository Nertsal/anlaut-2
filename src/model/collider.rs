use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Collider {
    /// An Axis-Aligned Bounding Box
    Aabb { size: Vec2<Coord> },
}

impl Collider {
    pub fn check(&self, other: &Self, delta_pos: Vec2<Coord>) -> bool {
        match (self, other) {
            (Collider::Aabb { size: size_a }, Collider::Aabb { size: size_b }) => {
                let a = AABB::ZERO.extend_symmetric(*size_a / Coord::new(2.0));
                let b = AABB::point(delta_pos).extend_symmetric(*size_b / Coord::new(2.0));
                a.x_min < b.x_max && a.x_max > b.x_min && a.y_min < b.y_max && a.y_max > b.y_min
            }
        }
    }
}

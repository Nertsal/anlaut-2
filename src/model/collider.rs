use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Collider {
    /// An Axis-Aligned Bounding Box
    Aabb { size: Vec2<Coord> },
}

impl Collider {
    pub fn check(&self, other: &Self, delta_pos: Position) -> bool {
        match (self, other) {
            (Collider::Aabb { size: size_a }, Collider::Aabb { size: size_b }) => {
                let a = AABB::ZERO.extend_positive(*size_a);
                let b = AABB::point(delta_pos).extend_positive(*size_b);
                a.x_min < b.x_max && a.x_max > b.x_min && a.y_min < b.y_max && a.y_max > b.y_min
            }
        }
    }
}

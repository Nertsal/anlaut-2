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
                let a = AABB::ZERO.extend_symmetric(*size_a / Coord::new(2.0));
                let b = AABB::point(delta_pos).extend_symmetric(*size_b / Coord::new(2.0));
                a.x_min < b.x_max && a.x_max > b.x_min && a.y_min < b.y_max && a.y_max > b.y_min
            }
        }
    }

    pub fn check_boundary(
        &self,
        position: Vec2<Coord>,
        boundary: AABB<Coord>,
    ) -> Option<Vec2<Coord>> {
        match self {
            Collider::Aabb { size } => {
                let aabb = AABB::point(position).extend_symmetric(*size / Coord::new(2.0));
                let dx_max = (aabb.x_max - boundary.x_max).max(Coord::ZERO);
                let dx_min = (aabb.x_min - boundary.x_min).min(Coord::ZERO);

                let dy_max = (aabb.y_max - boundary.y_max).max(Coord::ZERO);
                let dy_min = (aabb.y_min - boundary.y_min).min(Coord::ZERO);

                let dx = if dx_max.abs() > dx_min.abs() {
                    dx_max
                } else {
                    dx_min
                };
                let dy = if dy_max.abs() > dy_min.abs() {
                    dy_max
                } else {
                    dy_min
                };
                let d = vec2(dx, dy);
                if d == Vec2::ZERO {
                    None
                } else {
                    Some(d)
                }
            }
        }
    }
}

use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rotation {
    angle: R32,
}

impl Rotation {
    pub const ZERO: Self = Self { angle: R32::ZERO };

    pub fn new(angle: R32) -> Self {
        Self { angle }.normalized()
    }

    pub fn angle(&self) -> R32 {
        self.angle
    }

    pub fn direction(&self) -> Vec2<R32> {
        let (sin, cos) = self.angle.sin_cos();
        vec2(cos, sin)
    }

    pub fn normalized(mut self) -> Self {
        let two_pi = r32(2.0) * R32::PI;
        while self.angle < R32::ZERO {
            self.angle += two_pi;
        }
        while self.angle > two_pi {
            self.angle -= two_pi;
        }
        self
    }
}

impl std::ops::Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.angle + rhs.angle)
    }
}

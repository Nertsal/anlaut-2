use super::*;
use crate::model::{Coord, Time};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Diff)]
pub struct ServerAssets {
    pub config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Diff)]
pub struct Config {
    pub arena_size: Vec2<Coord>,

    pub singleplayer_humans: usize,
    pub multiplayer_humans_delta: usize,

    pub blocks_number: usize,
    pub block_min_size: Vec2<Coord>,
    pub block_max_size: Vec2<Coord>,
    pub blocks_spacing: Coord,

    pub human_knockout_time: Time,
    pub human_walk_speed: Coord,
    pub human_run_speed: Coord,
    pub human_turn_speed: Coord,

    pub gun_size: Vec2<Coord>,
    pub gun_shoot_speed: Coord,
    pub gun_recoil_speed: Coord,
    pub gun_heavy_recoil_speed: Coord,
    pub gun_recoil_attached_speed: Coord,
    pub gun_friction: Coord,
    pub gun_orbit_radius: Coord,
    pub gun_bounciness: Coord,

    pub projectile_lifetime: Time,
}

#[derive(geng::Assets)]
pub struct Assets {
    pub field: Rc<ugli::Program>,
}

impl Assets {
    pub async fn process(&mut self, _geng: &Geng) {}
}

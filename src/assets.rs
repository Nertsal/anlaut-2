use super::*;
use crate::model::{Coord, Score, Time};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Diff)]
pub struct ServerAssets {
    pub config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Diff)]
pub struct Config {
    pub round_time: Time,
    pub game_restart_delay: Time,
    pub game_initial_delay: Time,
    pub arena_size: Vec2<Coord>,

    pub singleplayer_humans: usize,
    pub multiplayer_humans_delta: usize,

    pub blocks_number: usize,
    pub block_min_size: Vec2<Coord>,
    pub block_max_size: Vec2<Coord>,
    pub blocks_spacing: Coord,

    pub human_kill_score: Score,
    pub gun_kill_score: Score,

    pub human_knockout_time: Time,
    pub human_walk_speed: Coord,
    pub human_run_speed: Coord,
    pub human_turn_speed: Coord,
    pub human_powerup_chance: R32,

    pub powerup_size: Vec2<Coord>,

    pub gun_reload_time: Time,
    pub gun_magazine_size: usize,
    pub gun_respawn_time: Time,

    pub gun_size: Vec2<Coord>,
    pub gun_shoot_speed: Coord,
    pub gun_shoot_lifetime: Time,
    pub gun_recoil_speed: Coord,
    pub gun_recoil_attached_speed: Coord,

    pub gun_heavy_speed: Coord,
    pub gun_heavy_lifetime: Time,
    pub gun_heavy_recoil_speed: Coord,
    pub gun_heavy_bullets: usize,
    pub gun_heavy_angle: R32,

    pub gun_friction: Coord,
    pub gun_orbit_radius: Coord,
    pub gun_bounciness: Coord,
}

#[derive(geng::Assets)]
pub struct Assets {
    pub shaders: Shaders,
    pub shoot: geng::Sound,
    pub hit: geng::Sound,
}

#[derive(geng::Assets)]
pub struct Shaders {
    pub field: Rc<ugli::Program>,
    pub post: Rc<ugli::Program>,
}

impl Assets {
    pub async fn process(&mut self, _geng: &Geng) {}
}

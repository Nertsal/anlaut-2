use super::*;

mod collider;
mod id;
mod update;

pub use collider::*;
pub use id::*;

pub const GUN_SIZE: Vec2<f32> = vec2(2.0, 1.0);
pub const GUN_SHOOT_SPEED: f32 = 5.0;

pub type Time = R32;
pub type Coord = R32;
pub type Position = Vec2<Coord>;

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq)]
pub struct Model {
    id_gen: IdGen,
    players: Collection<Player>,
    pub humans: Collection<Human>,
    pub guns: Collection<Gun>,
    pub projectiles: Collection<Projectile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState {
    Lobby,
    Gun { gun_id: Id },
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Player {
    pub id: PlayerId,
    #[diff = "eq"]
    pub state: PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Human {
    pub id: Id,
    pub position: Position,
    #[diff = "eq"]
    pub collider: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Gun {
    pub id: Id,
    pub position: Position,
    #[diff = "eq"]
    pub collider: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize, Diff, PartialEq, Eq, HasId)]
pub struct Projectile {
    pub id: Id,
    pub position: Position,
    pub velocity: Vec2<Coord>,
    #[diff = "eq"]
    pub collider: Collider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Shoot { direction: Vec2<Coord> },
    SpawnHuman { position: Position },
    SpawnGun { position: Position },
}

pub type Event = ();

impl Model {
    pub fn new() -> Self {
        Self {
            id_gen: IdGen::new(),
            players: default(),
            humans: default(),
            guns: default(),
            projectiles: default(),
        }
    }
}

impl net::Model for Model {
    type PlayerId = PlayerId;
    type Message = Message;
    type Event = Event;
    const TICKS_PER_SECOND: f32 = TICKS_PER_SECOND;

    fn new_player(&mut self, _events: &mut Vec<Self::Event>) -> Self::PlayerId {
        let gun_id = self.id_gen.next();
        let mut rng = global_rng();
        let position = vec2(rng.gen_range(-5.0..=5.0), rng.gen_range(-5.0..=5.0)).map(Coord::new);
        let gun = Gun {
            id: gun_id,
            position,
            collider: Collider::Aabb {
                size: GUN_SIZE.map(Coord::new),
            },
        };
        self.guns.insert(gun);

        let id = self.id_gen.next_player();
        let player = Player {
            id,
            state: PlayerState::Gun { gun_id },
        };
        self.players.insert(player);
        id
    }

    fn drop_player(&mut self, _events: &mut Vec<Self::Event>, player_id: &Self::PlayerId) {
        let _player = self.players.remove(player_id);
    }

    fn handle_message(
        &mut self,
        _events: &mut Vec<Self::Event>,
        player_id: &Self::PlayerId,
        message: Self::Message,
    ) {
        match message {
            Message::Shoot { direction } => {
                if let Some(player) = self.players.get(player_id) {
                    if let PlayerState::Gun { gun_id } = &player.state {
                        if let Some(gun) = self.guns.get(gun_id) {
                            let projectile = Projectile {
                                id: self.id_gen.next(),
                                position: gun.position,
                                velocity: direction.normalize_or_zero()
                                    * Coord::new(GUN_SHOOT_SPEED),
                                collider: Collider::Aabb {
                                    size: vec2(1.0, 1.0).map(Coord::new),
                                },
                            };
                            self.projectiles.insert(projectile);
                        }
                    }
                }
            }
            Message::SpawnHuman { position } => {
                let human = Human {
                    id: self.id_gen.next(),
                    position,
                    collider: Collider::Aabb {
                        size: vec2(2.0, 2.0).map(Coord::new),
                    },
                };
                self.humans.insert(human);
            }
            Message::SpawnGun { position } => {
                let gun = Gun {
                    id: self.id_gen.next(),
                    position,
                    collider: Collider::Aabb {
                        size: vec2(2.0, 1.0).map(Coord::new),
                    },
                };
                self.guns.insert(gun);
            }
        }
    }

    fn tick(&mut self, _events: &mut Vec<Self::Event>) {
        let delta_time = Time::ONE / Time::new(Self::TICKS_PER_SECOND);
        self.update(delta_time);
    }
}

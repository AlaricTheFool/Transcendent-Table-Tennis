use crate::prelude::*;

const BALL_RADIUS: f32 = 16.;
const PADDLE_WIDTH: f32 = 16.;
const PADDLE_HEIGHT: f32 = 128.;

pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ball {
    pub radius: f32,
    pub velocity: Vec2
}

impl Ball {
    pub fn new() -> Self {
        Self {
            radius: BALL_RADIUS,
            velocity: Vec2::ZERO
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paddle {
    pub side: Side,
    pub up_key: KeyCode,
    pub down_key: KeyCode,
    pub size: Vec2,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        let (up_key, down_key) = match side {
            Side::Left => {
                (KeyCode::W, KeyCode::S)
            },
            Side::Right => {
                (KeyCode::Up, KeyCode::Down)
            }
        };
        
        Self {
            side,
            up_key,
            down_key,
            size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        }
    }

    pub fn convert_offset_to_percentage(&self, offset: f32) -> f32 {
        let max_offset = self.size.y / 2.0;
        offset / max_offset
    }
}

pub struct BallCollision {
    pub ball: Entity,
    pub vert_velocity: f32,
}

pub struct CreateParticleBurstMessage {
    pub position: Vec2,
}

pub struct RenderRectangle {
    pub size: Vec2,
    pub color: Color,
}

pub struct Particle{
    pub dir: Vec2,
}

impl Particle {
    pub fn random_dir() -> Self {
        Self {
            dir: Vec2::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0))
        }
    }
}

pub struct PlayerController {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

pub struct AIController;

pub struct PaddleMove {
    pub paddle: Entity,
    pub dir: f32,
}

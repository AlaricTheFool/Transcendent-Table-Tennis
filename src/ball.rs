use crate::prelude::*;

pub const BALL_RADIUS: f32 = 16.;

const SPEED_INCREASE_PER_BOUNCE: f32 = 2.;
const MAX_H_SPEED: f32 = BALL_RADIUS * 2.;

pub struct Ball {
    pub pos: Vec2,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: center_screen(),
            velocity: Self::random_initial_velocity(),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, BALL_RADIUS, BALL_COLOR);
    }

    pub fn update(&mut self) {
        self.pos += self.velocity;

        if (self.pos.y + BALL_RADIUS) > screen_height() || (self.pos.y - BALL_RADIUS) < 0. {
            self.velocity.y *= -1.;
            self.pos.y = self.pos.y.clamp(0.0 + BALL_RADIUS, screen_height() - 16.0); 
        } 
    }

    pub fn is_off_screen(&self) -> Option<Side> {
        if self.pos.x < 0.0 - BALL_RADIUS || self.pos.x > screen_width() + BALL_RADIUS {
            if self.pos.x < 0.0 {
                Some(Side::Left)
            } else {
                Some(Side::Right)
            }
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.pos = center_screen();
        self.velocity = Self::random_initial_velocity();
    }

    pub fn bounce(&mut self, vert_velocity: f32) {
        let current_speed = self.velocity.x.abs();
        self.velocity.x = (current_speed + SPEED_INCREASE_PER_BOUNCE).min(MAX_H_SPEED) * -self.velocity.x.signum();
        self.velocity.y = vert_velocity;
    }
}

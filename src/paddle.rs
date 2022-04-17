use crate::prelude::*;

const PADDLE_WIDTH: f32 = 16.;
const PADDLE_HEIGHT: f32 = 128.;
const PADDLE_WINDOW_EDGE_OFFSET: f32 = 16.;
const PADDLE_SPEED: f32 = 10.0;

pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    side: Side,
    pos: Vec2,
    up_key: KeyCode,
    down_key: KeyCode,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        let (x_pos, up_key, down_key) = match side {
            Side::Left => {
                (
                    PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH / 2.0),
                    KeyCode::W,
                    KeyCode::S,
                )
            },

            Side::Right => {
                (
                    screen_width() - (PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH / 2.0)),
                    KeyCode::Up,
                    KeyCode::Down,
                )
            },
        };

        Paddle {
            up_key,
            down_key,
            side,
            pos: Vec2::new(x_pos, screen_height() / 2.0),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - (PADDLE_WIDTH * 0.5), 
            self.pos.y - (PADDLE_HEIGHT * 0.5), 
            PADDLE_WIDTH, 
            PADDLE_HEIGHT, 
            PADDLE_COLOR)
    }

    pub fn does_collide_with(&self, ball: &Ball) -> Option<f32> {
        let correct_dir: bool = match self.side {
            Side::Left => {
                ball.velocity.x < 0.
            },

            Side::Right => {
                ball.velocity.x > 0.
            },
        };

        let y_aligned = (ball.pos.y + BALL_RADIUS) >= self.pos.y - (PADDLE_HEIGHT / 2.0) 
                        && (ball.pos.y - BALL_RADIUS) <= self.pos.y + (PADDLE_HEIGHT / 2.0);
        
        let x_aligned = (ball.pos.x + BALL_RADIUS) >= self.pos.x - (PADDLE_WIDTH / 2.0) 
                        && (ball.pos.x - BALL_RADIUS) <= self.pos.x + (PADDLE_WIDTH / 2.0);

        if correct_dir && x_aligned && y_aligned {
            Some(self.calculate_ball_bounce_dir(ball))
        }
        else {
            None
        }
    }

    pub fn update(&mut self) {
        let is_up = is_key_down(self.up_key);
        let is_down = is_key_down(self.down_key);
        
        let paddle_dir = match (is_up, is_down) {
            (true, false) => -1.0, // up
            (false, true) => 1.0,  // down
            (_, _) => 0.0          // up and down or neither
        };

        self.pos.y += paddle_dir * PADDLE_SPEED;

        self.pos.y = self.pos.y.clamp(0.0 + (PADDLE_HEIGHT / 2.0), screen_height() - (PADDLE_HEIGHT / 2.0));
    }

    fn calculate_ball_bounce_dir(&self, ball: &Ball) -> f32 {
        let ball_v_offset = (ball.pos.y - self.pos.y).abs();
        let bounce_dir = (ball.pos.y - self.pos.y).signum();

        if ball_v_offset < Self::convert_percentage_to_max_offset(STRAIGHT_SHOT_AREA_PCT) {
            0.
        } else if ball_v_offset < Self::convert_percentage_to_max_offset(STRAIGHT_SHOT_AREA_PCT + SHALLOW_ANGLE_AREA_PCT){
            SHALLOW_ANGLE_VELOCITY * bounce_dir
        } else {
            DEEP_ANGLE_VELOCITY * bounce_dir
        }
    }

    fn convert_percentage_to_max_offset(pct: f32) -> f32 {
        (pct / 2.) * PADDLE_HEIGHT
    }
}

const DEEP_ANGLE_VELOCITY: f32 = 7.;
const SHALLOW_ANGLE_VELOCITY: f32 = 3.5;

// You could NOT use DEEP_BANK_SHOT since it's always going to be the else condition.
const STRAIGHT_SHOT_AREA_PCT: f32 = 0.10;
const DEEP_BANK_SHOT_AREA_PCT: f32 = 0.20;
const SHALLOW_ANGLE_AREA_PCT: f32 = 1.0 - STRAIGHT_SHOT_AREA_PCT - DEEP_BANK_SHOT_AREA_PCT;


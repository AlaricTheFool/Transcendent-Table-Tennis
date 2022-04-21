use crate::prelude::*;

const PADDLE_WIDTH: f32 = 16.;
const PADDLE_HEIGHT: f32 = 128.;
const PADDLE_WINDOW_EDGE_OFFSET: f32 = 16.;
const PADDLE_SPEED: f32 = 10.0;

#[system(for_each)]
pub fn reset_paddle(pos: &mut Vec2, paddle: &Paddle) {
    let x_pos = match paddle.side {
        Side::Left => {
            PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH / 2.0)
        },

        Side::Right => {
            screen_width() - (PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH / 2.0))
        },
    };
    
    *pos = Vec2::new(x_pos, screen_height() / 2.0);
}

#[system(for_each)]
pub fn move_paddle(pos: &mut Vec2, paddle: &Paddle) {
    let is_up = is_key_down(paddle.up_key);
    let is_down = is_key_down(paddle.down_key);
    
    let paddle_dir = match (is_up, is_down) {
        (true, false) => -1.0, // up
        (false, true) => 1.0,  // down
        (_, _) => 0.0          // up and down or neither
    };

    pos.y += paddle_dir * PADDLE_SPEED;

    pos.y = pos.y.clamp(0.0 + (PADDLE_HEIGHT / 2.0), screen_height() - (PADDLE_HEIGHT / 2.0));
}

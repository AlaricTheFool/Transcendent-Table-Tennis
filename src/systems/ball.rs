use crate::prelude::*;

const MAX_INITIAL_V_SPEED: f32 = 5.;
const INITIAL_BALL_SPEED: f32 = 5.;

#[system(for_each)]
pub fn draw_ball(pos: &Vec2, ball: &Ball, #[resource] cam: &PongCam) {
    draw_circle(pos.x - cam.offset.x, pos.y - cam.offset.y, ball.radius, BALL_COLOR);
}

#[system(for_each)]
pub fn serve_ball(pos: &mut Vec2, ball: &mut Ball, #[resource] match_state: &mut MatchState) {
    //TODO: Add a delay before launch.
    *pos = center_screen();
    ball.velocity = Vec2::new(if thread_rng().gen::<bool>() { INITIAL_BALL_SPEED } else { -INITIAL_BALL_SPEED }, thread_rng().gen_range(-MAX_INITIAL_V_SPEED..MAX_INITIAL_V_SPEED));
    *match_state = MatchState::BallInPlay;
}

#[system(for_each)]
pub fn ball_movement(pos: &mut Vec2, ball: &mut Ball) {
    *pos += ball.velocity;

    if (pos.y + ball.radius) > screen_height() || (pos.y - ball.radius) < 0. {
        ball.velocity.y *= -1.;
        pos.y = pos.y.clamp(0.0 + ball.radius, screen_height() - ball.radius); 
    } 
}

#[system(for_each)]
pub fn ball_scoring(pos: &Vec2, ball: &Ball, #[resource] match_state: &mut MatchState) {
    if let Some(side) = is_ball_off_screen(*ball, *pos) {
        //TODO: More elaborate scoring code
        *match_state = MatchState::BallToServe
    }
}

fn is_ball_off_screen(ball: Ball, pos: Vec2) -> Option<Side> {
    if pos.x < 0.0 - ball.radius || pos.x > screen_width() + ball.radius {
        if pos.x < 0.0 {
            Some(Side::Left)
        } else {
            Some(Side::Right)
        }
    } else {
        None
    }
}

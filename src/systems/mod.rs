mod ball;
mod background;
mod paddle;
mod collisions;

use crate::prelude::*;

/*
pub fn match_render_schedule_builder() -> Builder {
    Schedule::builder()
        .add_thread_local(background::draw_background_system())
        .add_thread_local(ball::draw_ball_system())
        .build()
}
*/

pub fn build_ball_in_play_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(background::draw_background_system())
        .add_thread_local(ball::draw_ball_system())
        .add_thread_local(paddle::draw_paddle_system())
        .flush()
        .add_system(paddle::move_paddle_system())
        .add_system(ball::ball_movement_system())
        .add_system(collisions::create_collisions_system())
        .flush()
        .add_system(collisions::bounce_balls_system())
        .flush()
        .add_system(ball::ball_scoring_system())
        .build()
}

pub fn build_ball_to_serve_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(background::draw_background_system())
        .add_thread_local(ball::draw_ball_system())
        .add_thread_local(paddle::draw_paddle_system())
        .flush()
        .add_system(paddle::reset_paddle_system())
        .add_system(ball::serve_ball_system())
        .build()
}

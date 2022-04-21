mod ball;
mod background;
mod paddle;
mod collisions;
mod particles;
mod drawing;
mod shake_screen;

use crate::prelude::*;

pub fn build_ball_in_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(shake_screen::shake_screen_system())
        .flush()
        .add_thread_local(background::draw_background_system())
        .add_thread_local(ball::draw_ball_system())
        .add_thread_local(drawing::draw_rectangles_system())
        .flush()
        .add_system(particles::move_particles_system())
        .add_system(paddle::move_paddle_system())
        .add_system(ball::ball_movement_system())
        .add_system(collisions::create_collisions_system())
        .flush()
        .add_system(collisions::bounce_balls_system())
        .add_system(particles::create_particles_system())
        .flush()
        .add_system(ball::ball_scoring_system())
        .build()
}

pub fn build_ball_to_serve_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(background::draw_background_system())
        .add_thread_local(ball::draw_ball_system())
        .add_thread_local(drawing::draw_rectangles_system())
        .flush()
        .add_system(paddle::reset_paddle_system())
        .add_system(particles::clear_particles_system())
        .add_system(ball::serve_ball_system())
        .build()
}

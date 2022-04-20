use crate::prelude::*;

pub fn spawn_ball(ecs: &mut World) {
    ecs.push(
        (
            center_screen(),
            Ball::new(),
        )
    );
}

pub fn spawn_paddles(ecs: &mut World) {
    ecs.push(
        (
            center_screen(),
            Paddle::new(Side::Left),
        )
    );

    ecs.push(
        (
            center_screen(),
            Paddle::new(Side::Right),
        )
    );
}

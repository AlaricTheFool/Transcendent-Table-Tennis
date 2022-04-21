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
            RenderRectangle{
                color: PADDLE_COLOR,
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
            },
            PlayerController {
                up_key: KeyCode::W,
                down_key: KeyCode::S,
            }
        )
    );

    ecs.push(
        (
            center_screen(),
            Paddle::new(Side::Right),
            RenderRectangle{
                color: PADDLE_COLOR,
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
            },
            AIController
        )
    );
}

pub fn spawn_particle_burst(commands: &mut CommandBuffer, pos: Vec2) {
    (0..100).for_each(|_| {
        spawn_particle(commands, pos);
    }); 
}

fn spawn_particle(commands: &mut CommandBuffer, pos: Vec2) {
    commands.push(
        (
            Particle::random_dir(),
            RenderRectangle{
                color: BALL_COLOR,
                size: Vec2::ONE * thread_rng().gen_range(1.0..3.0),
            },
            pos,
        )
    );
}

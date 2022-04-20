use crate::prelude::*;

#[system]
#[read_component(Ball)]
#[read_component(Paddle)]
#[read_component(Vec2)]
pub fn create_collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut balls = <(Entity, &Ball, &Vec2)>::query();
    let mut paddles = <(Entity, &Paddle, &Vec2)>::query();

    balls.iter(ecs).for_each(|(b_entity, ball, ball_pos)| {
            paddles.iter(ecs).for_each(|(p_entity, paddle, paddle_pos)| {
                let correct_dir: bool = match paddle.side {
                    Side::Left => {
                        ball.velocity.x < 0.
                    },
                
                    Side::Right => {
                        ball.velocity.x > 0.
                    },
                };
    
                let y_aligned = (ball_pos.y + ball.radius) >= paddle_pos.y - (paddle.size.y / 2.0) 
                && (ball_pos.y - ball.radius) <= paddle_pos.y + (paddle.size.y / 2.0);
    
                let x_aligned = (ball_pos.x + ball.radius) >= paddle_pos.x - (paddle.size.x / 2.0) 
                            && (ball_pos.x - ball.radius) <= paddle_pos.x + (paddle.size.x / 2.0);
    
                
                if correct_dir && x_aligned && y_aligned {
                    commands.push(((), BallCollision{ ball: *b_entity, vert_velocity: calculate_ball_bounce_dir(*paddle, *paddle_pos, *ball_pos)}));
                    commands.push(((), CreateParticleBurstMessage{ position: *ball_pos }));

                }
            });
    }); 
}

#[system(for_each)]
#[write_component(Ball)]
pub fn bounce_balls(entity: &Entity, collision: &BallCollision, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let ball_entity = ecs.entry_ref(collision.ball).unwrap();
    let ball = ball_entity.get_component::<Ball>().unwrap();
    let new_ball = Ball {
        radius: ball.radius,
        velocity: create_bounced_vector(ball.velocity, collision.vert_velocity),
    };

    commands.add_component(collision.ball, new_ball);
    

    commands.remove(*entity);
}

fn calculate_ball_bounce_dir(paddle: Paddle, paddle_pos: Vec2, ball_pos: Vec2) -> f32 {
    let ball_v_offset = (ball_pos.y - paddle_pos.y).abs();
    let bounce_dir = (ball_pos.y - paddle_pos.y).signum();

    if ball_v_offset < paddle.convert_percentage_to_max_offset(STRAIGHT_SHOT_AREA_PCT) {
        0.
    } else if ball_v_offset < paddle.convert_percentage_to_max_offset(STRAIGHT_SHOT_AREA_PCT + SHALLOW_ANGLE_AREA_PCT){
        SHALLOW_ANGLE_VELOCITY * bounce_dir
    } else {
        DEEP_ANGLE_VELOCITY * bounce_dir
    }
}

fn create_bounced_vector(speed: Vec2, vert_velocity: f32) -> Vec2 {
    let mut new_speed = speed;
    new_speed.x = (speed.x.abs() + SPEED_INCREASE_PER_BOUNCE).min(MAX_H_SPEED) * -speed.x.signum();
    new_speed.y = vert_velocity;
    new_speed
}

const DEEP_ANGLE_VELOCITY: f32 = 7.;
const SHALLOW_ANGLE_VELOCITY: f32 = 3.5;

// You could NOT use DEEP_BANK_SHOT since it's always going to be the else condition.
const STRAIGHT_SHOT_AREA_PCT: f32 = 0.10;
const DEEP_BANK_SHOT_AREA_PCT: f32 = 0.20;
const SHALLOW_ANGLE_AREA_PCT: f32 = 1.0 - STRAIGHT_SHOT_AREA_PCT - DEEP_BANK_SHOT_AREA_PCT;

const SPEED_INCREASE_PER_BOUNCE: f32 = 2.;

//TODO: The 16.0 is Ball Radius which is so cursed to have in this spot please put in a better place.
const MAX_H_SPEED: f32 = 16.0 * 2.;

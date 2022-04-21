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
pub fn control_player_paddles(
    entity: &Entity, 
    pos: &mut Vec2, 
    paddle: &Paddle, 
    controller: &PlayerController, 
    commands: &mut CommandBuffer
) {
    let is_up = is_key_down(controller.up_key);
    let is_down = is_key_down(controller.down_key);
    
    let paddle_dir = match (is_up, is_down) {
        (true, false) => -1.0, // up
        (false, true) => 1.0,  // down
        (_, _) => 0.0          // up and down or neither
    };

    commands.push(((), PaddleMove{ paddle: *entity, dir: paddle_dir }));
}

#[system]
#[read_component(Vec2)]
#[read_component(Paddle)]
#[read_component(AIController)]
#[read_component(Ball)]
pub fn control_ai_paddle(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut ai_paddles = <(Entity, &Vec2, &Paddle, &AIController)>::query();
    let mut balls = <(&Vec2, &Ball)>::query();

    let (ball_pos, _) = balls.iter(ecs).nth(0).unwrap();
    
    ai_paddles.iter(ecs).for_each(|(entity, pos, _, _)| {
        if (ball_pos.y - pos.y).abs() > 11.0 {
            let dir = (ball_pos.y - pos.y).signum();
            commands.push(((), PaddleMove{ paddle: *entity, dir }));
        }
    });
}

#[system(for_each)]
#[read_component(Paddle)]
#[write_component(Vec2)]
pub fn move_paddles(entity: &Entity, move_message: &PaddleMove, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    if let Ok(paddle_pos) = ecs
        .entry_mut(move_message.paddle)
        .unwrap()
        .get_component_mut::<Vec2>()
        {
            paddle_pos.y += move_message.dir * PADDLE_SPEED;
            paddle_pos.y = paddle_pos.y.clamp(0.0 + (PADDLE_HEIGHT / 2.0), screen_height() - (PADDLE_HEIGHT / 2.0));
        }
    
    commands.remove(*entity)
}

use macroquad::prelude::*;

const BG_COLOR:     Color = Color::new(031. / 255., 039. / 255., 027. / 255., 1.0);
const PADDLE_COLOR: Color = Color::new(052. / 255., 127. / 255., 196. / 255., 1.0);
const BALL_COLOR:   Color = Color::new(204. / 255., 041. / 255., 054. / 255., 1.0);

const BALL_RADIUS: f32 = 16.;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

struct Ball {
    pos: Vec2,
    velocity: Vec2,
}

impl Ball {
    fn new() -> Self {
        Self {
            pos: center_screen(),
            velocity: Vec2::ZERO,
        }
    }
}

fn center_screen() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

struct PongMatch {
    ball: Ball,
    left_paddle_pos: Vec2,
    right_paddle_pos: Vec2,
}

const PADDLE_WINDOW_EDGE_OFFSET: f32 = 16.;

impl PongMatch {
    fn new() -> Self {
        Self {
            ball: Ball::new(),
            left_paddle_pos: Vec2::new(PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH * 0.5), screen_height() / 2.0),
            right_paddle_pos: Vec2::new(screen_width() - (PADDLE_WINDOW_EDGE_OFFSET + (PADDLE_WIDTH * 0.5)), screen_height() / 2.0),
        }
    }

    fn render(&self) {
        clear_background(BG_COLOR);

        draw_circle(self.ball.pos.x, self.ball.pos.y, BALL_RADIUS, BALL_COLOR);

        draw_paddle(self.left_paddle_pos);
        draw_paddle(self.right_paddle_pos);
    }
}

const PADDLE_WIDTH: f32 = 16.;
const PADDLE_HEIGHT: f32 = 128.;

fn draw_paddle(pos: Vec2) {
    draw_rectangle(
        pos.x - (PADDLE_WIDTH * 0.5), 
        pos.y - (PADDLE_HEIGHT * 0.5), 
        PADDLE_WIDTH, 
        PADDLE_HEIGHT, 
        PADDLE_COLOR)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Transcendent Table Tennis".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pong_match = PongMatch::new();

    loop { 
        pong_match.render();

        next_frame().await
    }
}

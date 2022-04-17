mod ball;
mod paddle;

mod prelude {
    pub use macroquad::prelude::*; 
    pub use ::rand::prelude::*;

    pub const BG_COLOR:     Color = Color::new(031. / 255., 039. / 255., 027. / 255., 1.0);
    pub const PADDLE_COLOR: Color = Color::new(052. / 255., 127. / 255., 196. / 255., 1.0);
    pub const BALL_COLOR:   Color = Color::new(204. / 255., 041. / 255., 054. / 255., 1.0);
    pub const TEXT_COLOR:   Color = Color::new(203. / 255., 185. / 255., 168. / 255., 1.0);

    pub const WINDOW_WIDTH: i32 = 1280;
    pub const WINDOW_HEIGHT: i32 = 720;

    pub fn center_screen() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }

    pub use crate::ball::*;
    pub use crate::paddle::*;
}

use prelude::*;

const SCORE_TEXT_TOP_OFFSET: f32 = 40.;
const SCORE_TEXT_H_OFFSET: f32 = 40.;

struct PongMatch {
    ball: Ball,
    left_paddle: Paddle,
    left_score: i32,
    right_paddle: Paddle,
    right_score: i32,
}

impl PongMatch {
    fn new() -> Self {
        Self {
            ball: Ball::new(),
            left_paddle: Paddle::new(Side::Left),
            right_paddle: Paddle::new(Side::Right),
            left_score: 0,
            right_score: 0,
        }
    }

    fn render(&self) {
        clear_background(BG_COLOR);

        self.ball.draw();

        self.left_paddle.draw();
        self.right_paddle.draw();

        draw_text(&format!("{}", self.left_score), SCORE_TEXT_H_OFFSET, SCORE_TEXT_TOP_OFFSET, 30.0, TEXT_COLOR);
        
        let (f_size, f_scale, _) = camera_font_scale(30.0);
        let right_score_text = &format!("{}", self.right_score);
        let r_text_dim = measure_text(right_score_text, Some(Font::default()), f_size, f_scale);
        draw_text(right_score_text, screen_width() - SCORE_TEXT_H_OFFSET - r_text_dim.width, SCORE_TEXT_TOP_OFFSET, 30.0, TEXT_COLOR)
    }

    fn update(&mut self) {
        self.ball.update();
        self.left_paddle.update();
        self.right_paddle.update();

        if let Some(vert_velocity) = self.right_paddle.does_collide_with(&self.ball) {
            self.ball.velocity.x *= -1.;
            self.ball.velocity.y = vert_velocity;
        }

        if let Some(vert_velocity) = self.left_paddle.does_collide_with(&self.ball) {
            self.ball.velocity.x *= -1.;
            self.ball.velocity.y = vert_velocity;
        }

        if let Some(side) = self.ball.is_off_screen() {
            self.ball.reset();

            match side {
                Side::Left => {
                    self.right_score += 1;
                }
                
                Side::Right => {
                    self.left_score += 1;
                }
            }
        }
    }
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
        pong_match.update(); 
        pong_match.render();

        next_frame().await
    }
}

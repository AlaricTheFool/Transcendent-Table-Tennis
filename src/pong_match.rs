use crate::prelude::*;

const SCORE_TEXT_TOP_OFFSET: f32 = 40.;
const SCORE_TEXT_H_OFFSET: f32 = 40.;

pub struct PongMatch {
    ball: Ball,
    left_paddle: Paddle,
    left_score: i32,
    right_paddle: Paddle,
    right_score: i32,
}

impl PongMatch {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            left_paddle: Paddle::new(Side::Left),
            right_paddle: Paddle::new(Side::Right),
            left_score: 0,
            right_score: 0,
        }
    }
}

impl GameState for PongMatch {
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
            self.ball.bounce(vert_velocity);
        }

        if let Some(vert_velocity) = self.left_paddle.does_collide_with(&self.ball) {
            self.ball.bounce(vert_velocity);
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

    fn next_state(&self) -> Option<GameStates> {
        if self.left_score >= 5 || self.right_score >= 5 {
            Some(GameStates::MainMenu)
        } else {
            None
        }
    }
}
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use macroquad::prelude::*; 
    pub use ::rand::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const BG_COLOR:     Color = Color::new(031. / 255., 039. / 255., 027. / 255., 1.0);
    pub const PADDLE_COLOR: Color = Color::new(052. / 255., 127. / 255., 196. / 255., 1.0);
    pub const BALL_COLOR:   Color = Color::new(204. / 255., 041. / 255., 054. / 255., 1.0);
    pub const TEXT_COLOR:   Color = Color::new(203. / 255., 185. / 255., 168. / 255., 1.0);

    pub const WINDOW_WIDTH: i32 = 1280;
    pub const WINDOW_HEIGHT: i32 = 720;

    pub fn center_screen() -> Vec2 {
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum MatchState {
        BallToServe,
        BallInPlay,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Side {
        Left,
        Right,
    }

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

/*
struct Game {
    current_state: GameStates,
    pong_match: PongMatch,
    main_menu: MainMenu,
}

impl Game {
    fn new() -> Self {
        Game {
            current_state: GameStates::PongMatch,
            pong_match: PongMatch::new(),
            main_menu: MainMenu::new(),
        }
    }

    fn tick(&mut self) {
        self.pong_match.update();
        self.pong_match.render();
    }
}
*/

struct State {
    world: World,
    resources: Resources,
    ball_in_play_schedule: Schedule,
    ball_to_serve_schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let mut resources = Resources::default();
        let mut world = World::default();

        resources.insert(MatchState::BallToServe);
    
        spawn_ball(&mut world);
        spawn_paddles(&mut world);

        Self{
            world,
            resources,
            ball_in_play_schedule: build_ball_in_play_schedule(),
            ball_to_serve_schedule: build_ball_to_serve_schedule(),
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

    let mut state = State::new();

    
    loop {
        let match_state = state.resources.get::<MatchState>().unwrap().clone();

        match match_state {
            MatchState::BallInPlay => {
                state.ball_in_play_schedule.execute(&mut state.world, &mut state.resources);
            }

            MatchState::BallToServe => {
                state.ball_to_serve_schedule.execute(&mut state.world, &mut state.resources);
            }
        }


        next_frame().await
    }
}

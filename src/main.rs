mod components;
mod spawner;
mod systems;
mod camera;

mod prelude {
    pub use macroquad::prelude::*; 
    pub use macroquad::audio::*;
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

    pub const PADDLE_WIDTH: f32 = 16.;
    pub const PADDLE_HEIGHT: f32 = 128.;    

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

    pub struct MatchSounds {
        pub bounce_sounds: [Sound; 3],
        pub score_sounds: [Sound; 3],
    }

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::camera::*;
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
        resources.insert(PongCam{ offset: Vec2::ZERO, shake_frames: 0 });

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

    set_pc_assets_folder("assets");

    let mut state = State::new();

    let mut bounce_sounds: [Sound; 3] = [load_sound("sfx/uninitialized_sound.wav").await.expect("Failed to load sound."); 3];
    for num in 1..=3 {
        bounce_sounds[num - 1] = load_sound(&format!("sfx/ball_bounce/bounce_{}.wav", num)).await.expect("Failed to load sound.");
    }

    let mut score_sounds: [Sound; 3] = [load_sound("sfx/uninitialized_sound.wav").await.expect("Failed to load sound."); 3];
    for num in 1..=3 {
        score_sounds[num - 1] = load_sound(&format!("sfx/score/score_{}.wav", num)).await.unwrap();
    }

    state.resources.insert(MatchSounds{
        bounce_sounds,
        score_sounds,
    });
    
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

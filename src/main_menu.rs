use crate::prelude::*;

pub struct MainMenu {
}

impl MainMenu {
    pub fn new() -> Self {
        Self{}
    }
}

impl GameState for MainMenu {
    fn render(&self) {
        clear_background(BG_COLOR);
    }
    
    fn update(&mut self) {}
    
    fn next_state(&self) -> Option<GameStates> {
        None
    }
}

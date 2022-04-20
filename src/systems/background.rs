use crate::prelude::*;

#[system]
pub fn draw_background() {
    clear_background(BG_COLOR);
}
use crate::prelude::*;

const MAX_SHAKE: f32 = 4.0;

#[system]
pub fn shake_screen(#[resource] cam: &mut PongCam) {
    if cam.shake_frames > 0 {
        cam.offset = Vec2::new(thread_rng().gen_range(-MAX_SHAKE..MAX_SHAKE), thread_rng().gen_range(-MAX_SHAKE..MAX_SHAKE));
        cam.shake_frames -= 1;
    } else {
        cam.offset = Vec2::ZERO;
    }
}
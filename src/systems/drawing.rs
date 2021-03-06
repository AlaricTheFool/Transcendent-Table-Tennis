use crate::prelude::*;

#[system]
#[read_component(Vec2)]
#[read_component(RenderRectangle)]
pub fn draw_rectangles(ecs: &SubWorld, #[resource] cam: &PongCam) {
    let mut query = <(&Vec2, &RenderRectangle)>::query();

    for (pos, rect) in query.iter(ecs) {
        draw_rectangle(
            //TODO: USE paddle size values
            pos.x - (rect.size.x * 0.5) + -cam.offset.x, 
            pos.y - (rect.size.y * 0.5) + -cam.offset.y, 
            rect.size.x, 
            rect.size.y, 
            rect.color)
    }
}
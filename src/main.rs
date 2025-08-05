use macroquad::prelude::*;  

#[macroquad::main("Chaikin's Algo")]
async fn main() {
    let mut arr = Vec::new();

    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let center = mouse_position();
            arr.push(center);
        }
        for x in &arr {
            draw_circle(x.0, x.1, 5.0, WHITE);
        }

        next_frame().await
    }
}
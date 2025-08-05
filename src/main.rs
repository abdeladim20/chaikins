use macroquad::prelude::*;  

#[macroquad::main("Chaikin's Algo")]
async fn main() {
    let mut control_points = Vec::new();
    let mut display_points = Vec::new();
    let mut is_animating = false;
    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            control_points.push(mouse_position());
        }
        if is_key_pressed(KeyCode::Enter) && control_points.len() >= 2 {
            is_animating = true;
            display_points = control_points.clone();
        }
        for x in &control_points {
            draw_circle(x.0, x.1, 5.0, WHITE);
        }
        if is_animating {
            for i in 0..control_points.len()-1 {
                draw_line(control_points[i].0, control_points[i].1, control_points[i+1].0, control_points[i+1].1, 1.0, WHITE);
            }
        } 

        next_frame().await
    }
}

// pub fn chaiken(points: Vec<f32, f32>) -> Vec {

// }
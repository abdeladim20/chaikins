use macroquad::prelude::*;

#[macroquad::main("Chaikin's Algo")]
async fn main() {
    let mut control_points = Vec::new();
    let mut display_points = Vec::new();
    let mut is_animating = false;
    let mut animation_step = 0;
    let mut last_update_time = 0.0;

    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) && !is_animating {
            control_points.push(mouse_position());
        }
        if is_key_pressed(KeyCode::Enter) && control_points.len() >= 2 {
            is_animating = true;
            animation_step = 0;
            last_update_time = get_time();
            display_points = control_points.clone();
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            is_animating = false;
            animation_step = 0;
            display_points.clear();
            control_points.clear();
            last_update_time = 0.0;
        }

        for x in &control_points {
            draw_circle(x.0, x.1, 5.0, WHITE);
        }

        if is_animating {
            if control_points.len() > 2 {
                if get_time() - last_update_time > 1.0 {
                    if animation_step < 7 {
                        display_points = chaiken(&display_points);
                        animation_step += 1;
                    } else {
                        animation_step = 0;
                        display_points = control_points.clone();
                    }
                    last_update_time = get_time();
                }
            }

            if display_points.len() > 1 {
                for points_pair in display_points.windows(2) {
                    let p1 = points_pair[0];
                    let p2 = points_pair[1];
                    draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, BLUE);
                }
            }
        }
        next_frame().await
    }
}

pub fn chaiken(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();
    new_points.push(points[0]);

    for points_pair in points.windows(2) {
        let p0 = points_pair[0];
        let p1 = points_pair[1];

        let distance_x = p1.0 - p0.0;
        let distance_y = p1.1 - p0.1;

        let q0_x = p0.0 + distance_x * 0.25;
        let q0_y = p0.1 + distance_y * 0.25;

        let q1_x = p0.0 + distance_x * 0.75;
        let q1_y = p0.1 + distance_y * 0.75;

        new_points.push((q0_x, q0_y));
        new_points.push((q1_x, q1_y));
    }
    new_points.push(points[points.len() - 1]);

    new_points
}

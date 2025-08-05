use macroquad::prelude::*;
use chaikin::chaikin_algo;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut control_points: Vec<(f32, f32)> = Vec::new();
    let mut steps: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut animating = false;
    let mut current_step = 0;
    let mut timer = 0.0;
    let step_duration = 0.5;
    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            control_points.push((mx, my));
        }

        if is_key_pressed(KeyCode::Enter) && control_points.len() >= 1 {
            steps.clear();
            steps.push(control_points.clone());
            let mut current_points = control_points.clone();
            for _ in 0..7 {
                current_points = chaikin_algo(&current_points);
                steps.push(current_points.clone());
            }
            animating = true;
            current_step = 0;
            timer = 0.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if animating {
            timer += get_frame_time();
            if timer >= step_duration {
                timer = 0.0;
                current_step += 1;
                if current_step >= steps.len() {
                    current_step = 0;
                }
            }
        }
        
        let mut points_to_draw = if animating {
            &steps[current_step]
        } else {
            &control_points
        };

        for &(x, y) in &control_points {
            draw_circle(x, y, 2.0, GRAY);
        }

        if !animating && control_points.len() >= 2 {
            for i in 0..control_points.len() - 1 {
                let (x1, y1) = control_points[i];
                let (x2, y2) = control_points[i + 1];
                draw_line(x1, y1, x2, y2, 2.0, WHITE);
            }
        }

        for &(x, y) in points_to_draw {
            draw_circle(x, y, 4.0, GRAY);
        }

        if animating {
            points_to_draw = &steps[current_step];
            if points_to_draw.len() >= 2 {
                for i in 0..points_to_draw.len() - 1 {
                    let (x1, y1) = points_to_draw[i];
                    let (x2, y2) = points_to_draw[i + 1];
                    draw_line(x1, y1, x2, y2, 2.0, DARKBLUE);
                }
            }
        }
      


        next_frame().await;
    }
}

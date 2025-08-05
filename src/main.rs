use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    // let mut first_point: Option<(f32, f32)> = None;
    // let mut second_point: Option<(f32, f32)> = None;
    let mut entered = false;
    loop {
        clear_background(BLACK);
        if !entered{
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                points.push((mx, my));
            }
        }
        
        // if is_mouse_button_pressed(MouseButton::Left)  {
        //     println!("i'm in");
        //     let (x, y) = mouse_position();
        //     let first_point ==
        //     points.push((x, y));
        // }

        for &(x, y) in &points {
            draw_circle(x, y, 3.0, GRAY);
            draw_circle(x, y, 1.0, BLACK);
        }
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        if is_key_pressed(KeyCode::Enter) {
            entered = true;
        }
        if entered{
            let mut is_curved = false;
            while !is_curved{
                let (start_x, start_y) = points[0];
                let (end_x,end_y) = points[points.len()-1];

                draw_line(start_x, start_y, points[1].0, points[1].1, 2.0, RED);
                if points.len()>3{
                    for i in 1..points.len() - 2 {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[i + 1];
                        draw_line(x1, y1, x2, y2, 2.0, RED);
                    }
                }
                draw_line(points[points.len()-2].0, points[points.len()-2].1, end_x, end_y, 2.0, RED);
                is_curved= true
            }
            if is_key_pressed(KeyCode::Delete){
                points.clear();
                entered=false;
                is_curved=false
            }
        }

        // ui.label(None, "Left");
        // if is_mouse_button_down(MouseButton::Right) {
        //     ui.label(None, "Right");
        // }
        // if is_mouse_button_down(MouseButton::Middle) {
        //     ui.label(None, "Middle");
        // }
        // draw_line(200.0, 500.0, 500.0, 200.0, 1.0, BLUE);
        // draw_line(500.0, 200.0, 40.0, 40.0, 1.0, BLUE);

        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        next_frame().await;
    }
}

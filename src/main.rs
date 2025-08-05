use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    loop {
        clear_background(BLACK);
        println!("sm");
                let mut i = 0;
            while is_mouse_button_down(MouseButton::Left)  && i <= 3 {
                println!("i'm in");
                let (x, y) = mouse_position();
                points.push((x, y));
                i += 1;
            }
        
        
        for &(x, y) in &points {
            draw_circle(x, y, 10.0, YELLOW);
        }

            // ui.label(None, "Left");
        // if is_mouse_button_down(MouseButton::Right) {
        //     ui.label(None, "Right");
        // }
        // if is_mouse_button_down(MouseButton::Middle) {
        //     ui.label(None, "Middle");
        // }
        draw_line(200.0, 500.0, 500.0, 200.0, 1.0, BLUE);
        draw_line(500.0, 200.0, 40.0, 40.0, 1.0, BLUE);

        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        next_frame().await
    }
}


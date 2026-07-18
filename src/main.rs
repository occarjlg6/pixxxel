use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use pixxxel::Canvas;

const CANVAS_SIZE: usize = 32;
const SCALE: usize = 16;
const WIDTH: usize = CANVAS_SIZE * SCALE;
const HEIGHT: usize = CANVAS_SIZE * SCALE;

fn main() {
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE, 0x000000);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = match Window::new(
        "Pixxxel - Create Pixel Game Assets",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ) {
        Ok(window) => window,
        Err(err) => {
            eprintln!("Err: {err}");
            return;
        }
    };

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        canvas.render_into(&mut buffer, SCALE);

        //Read mouse pos

        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            let canvas_pos = (
                (pos.0 / SCALE as f32) as usize,
                (pos.1 / SCALE as f32) as usize,
            );
            if window.get_mouse_down(MouseButton::Left) {
                let active_color = 0xFF4444;

                if let Err(err) = canvas.set(canvas_pos.0, canvas_pos.1, active_color) {
                    eprintln!("Err: {err:?}");
                    return;
                }
            } else if window.get_mouse_down(MouseButton::Right) {
                let erase_color = 0x000000;

                if let Err(err) = canvas.set(canvas_pos.0, canvas_pos.1, erase_color) {
                    eprintln!("Err: {err:?}");
                    return;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this differently
        if let Err(err) = window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
            eprintln!("Err: {err}");
            break;
        }
    }
}

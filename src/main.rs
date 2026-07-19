use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use pixxxel::{Canvas, Palette};

const CANVAS_SIZE: usize = 32;
const SCALE: usize = 16;
const WIDTH: usize = CANVAS_SIZE * SCALE;
const HEIGHT: usize = CANVAS_SIZE * SCALE;

const PALETTE_KEYS: [Key; 8] = [
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
];

fn main() {
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE, 0x000000);
    let mut palette = Palette::new();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = match Window::new(
        format!(
            "Pixxxel - LMB={}, RMB={}",
            palette.primary_color_name(),
            palette.secondary_color_name()
        )
        .as_str(),
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

        // Mousepos
        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            let canvas_pos = (
                (pos.0 / SCALE as f32) as usize,
                (pos.1 / SCALE as f32) as usize,
            );
            if window.get_mouse_down(MouseButton::Left) {
                if let Err(err) = canvas.set(canvas_pos.0, canvas_pos.1, palette.primary_color()) {
                    eprintln!("Err: {err:?}");
                    return;
                }
            } else if window.get_mouse_down(MouseButton::Right) {
                if let Err(err) = canvas.set(canvas_pos.0, canvas_pos.1, palette.secondary_color())
                {
                    eprintln!("Err: {err:?}");
                    return;
                }
            }
        }

        // Number keys pick the primary color, Shift+number picks the secondary.
        let shift = window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);

        for idx in 0..PALETTE_KEYS.len() {
            if window.is_key_pressed(PALETTE_KEYS[idx], KeyRepeat::No) {
                let result = if shift {
                    palette.set_secondary_index(idx)
                } else {
                    palette.set_primary_index(idx)
                };

                if let Err(err) = result {
                    eprintln!("Err: {err:?}");
                }

                window.set_title(
                    format!(
                        "Pixxxel - LMB={}, RMB={}",
                        palette.primary_color_name(),
                        palette.secondary_color_name()
                    )
                    .as_str(),
                );
                break;
            }
        }

        // Save as .png
        if window.is_key_pressed(Key::E, KeyRepeat::No) {
            let file = rfd::FileDialog::new()
                .add_filter("images", &["png"])
                .set_file_name("canvas.png");

            if let Some(path) = file.save_file() {
                if let Err(err) = canvas.export_png(&path) {
                    eprintln!("Err: {err}");
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

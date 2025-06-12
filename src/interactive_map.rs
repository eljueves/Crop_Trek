use crate::file_io::OutputObject;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub fn run_interactive_map_view(tracked_objects: &Vec<OutputObject>) {
    let mut window = Window::new("Interactive Tracker Map", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Unable to create window");

    let mut buffer = vec![0x000000; WIDTH * HEIGHT];
    let mut viewport_x = 0.25f64;
    let mut viewport_y = 0.25f64;
    let viewport_w = 0.5f64;
    let viewport_h = 0.5f64;
    let move_step = 0.02;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) { viewport_x -= move_step; }
        if window.is_key_down(Key::Right) { viewport_x += move_step; }
        if window.is_key_down(Key::Up) { viewport_y -= move_step; }
        if window.is_key_down(Key::Down) { viewport_y += move_step; }

        buffer.iter_mut().for_each(|pixel| *pixel = 0x111111);

        let px = |xf: f64| -> usize { (xf * WIDTH as f64) as usize };
        let py = |yf: f64| -> usize { (yf * HEIGHT as f64) as usize };

        let x0 = px(viewport_x);
        let y0 = py(viewport_y);
        let x1 = px(viewport_x + viewport_w);
        let y1 = py(viewport_y + viewport_h);

        for x in x0..x1 {
            if y0 < HEIGHT { buffer[y0 * WIDTH + x] = 0xFFFFFF; }
            if y1 < HEIGHT { buffer[y1 * WIDTH + x] = 0xFFFFFF; }
        }
        for y in y0..y1 {
            if x0 < WIDTH { buffer[y * WIDTH + x0] = 0xFFFFFF; }
            if x1 < WIDTH { buffer[y * WIDTH + x1] = 0xFFFFFF; }
        }

        for det in tracked_objects {
            let dx = det.x;
            let dy = det.y;
            if dx >= viewport_x && dx <= viewport_x + viewport_w &&
               dy >= viewport_y && dy <= viewport_y + viewport_h {
                let sx = px((dx - viewport_x) / viewport_w);
                let sy = py((dy - viewport_y) / viewport_h);
                let sw = (det.width * WIDTH as f64 / viewport_w) as usize;
                let sh = (det.height * HEIGHT as f64 / viewport_h) as usize;
                for y in sy.saturating_sub(sh/2)..(sy + sh/2).min(HEIGHT) {
                    for x in sx.saturating_sub(sw/2)..(sx + sw/2).min(WIDTH) {
                        buffer[y * WIDTH + x] = 0x00FF00;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
use piston_window::*;
use chip8::{cpu::*, display};
use std::env;
use std::fs::File;
use std::io::Read;

const WINDOW_SCALE: usize = 20;
const WINDOW_DIMENSIONS: [u32; 2] = [(display::WIDTH * WINDOW_SCALE) as u32, (display::HEIGHT * WINDOW_SCALE) as u32];

fn main() {
    let path = env::args().nth(1).expect("Missing filename");
    let mut f = File::open(path).expect("Could not open file");
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).expect("Error reading file");
    let mut window: PistonWindow = WindowSettings::new("Chip 8 Emulator", WINDOW_DIMENSIONS)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut cpu = CPU::new(&buf);
    
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            draw_screen(&cpu.gfx.get_buffer(), &mut window, &e);
        }

        if let Some(u) = e.update_args() {
            cpu.run_cycle(u.dt);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if let Some(key_value) = key_value(&key) {
                cpu.key_release(key_value);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(key_value) = key_value(&key) {
                cpu.key_press(key_value);
            }
        }
    }
}

fn key_value(key: &Key) -> Option<u8> {
    if key.code() >= 48 && key.code() <= 57 {
        Some((key.code() - 48) as u8)
    } else if key.code() >= 97 && key.code() <= 102 {
        Some((key.code() - 97 + 10) as u8)
    } else {
        None
    }
}

fn draw_screen(display_buffer: &display::Buffer, window: &mut PistonWindow, e: &Event) {
    window.draw_2d(e, |context, graphics, _| {
        piston_window::clear(color::BLACK, graphics);

        for (i, row) in display_buffer.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val {
                    let dimensions = [(j * WINDOW_SCALE) as f64,
                                      (i * WINDOW_SCALE) as f64,
                                      WINDOW_SCALE as f64,
                                      WINDOW_SCALE as f64];
                    Rectangle::new(color::WHITE)
                        .draw(dimensions, &context.draw_state, context.transform, graphics);
                }
            }
        }
    });
}

use std::sync::mpsc;

use crate::memdsp::{Event, MemBuffer};
use minifb::{Key, Window, WindowOptions};

fn create_window(width: f32, height: f32, scale: f32) -> Window {
    Window::new(
        "Test - ESC to exit",
        (width * scale) as usize,
        (height * scale) as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window")
}

pub const zx_width: usize = 256;
pub const zx_height: usize = 192;
// ZX Spectrum Unbright Colors
const zx_unbright_colors: [u32; 8] = [
    0x000000, // Black
    0x0000C0, // Blue
    0xC00000, // Red
    0xC000C0, // Magenta
    0x00C000, // Green
    0x00C0C0, // Cyan
    0xC0C000, // Yellow
    0xC0C0C0, // White (light gray)
];
const zx_bright_colors: [u32; 8] = [
    0x000000, // Black (same in bright mode)
    0x0000FF, // Bright Blue
    0xFF0000, // Bright Red
    0xFF00FF, // Bright Magenta
    0x00FF00, // Bright Green
    0x00FFFF, // Bright Cyan
    0xFFFF00, // Bright Yellow
    0xFFFFFF, // Bright White
];

pub struct FBZXDisplay {
    window: Window,
    event_receiver: mpsc::Receiver<Event>,
    bitmap_buffer: MemBuffer,
    attribute_buffer: MemBuffer,
    should_close: bool,
}

impl FBZXDisplay {
    pub fn new(
        bitmap_buffer: MemBuffer,
        attribute_buffer: MemBuffer,
        scale: f32,
        event_receiver: mpsc::Receiver<Event>,
        refresh_rate: f64,
    ) -> FBZXDisplay {
        let mut window = create_window(zx_width as f32, zx_height as f32, scale);

        window.limit_update_rate(Some(std::time::Duration::from_micros(
            (1_000_000.0 / refresh_rate) as u64,
        )));
        FBZXDisplay {
            window,
            event_receiver,
            bitmap_buffer,
            attribute_buffer,
            should_close: false,
        }
    }

    fn handle_events(&mut self) {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            self.should_close = true;
        }
        for event in self.event_receiver.try_iter() {
            match event {
                Event::Exit => {
                    self.should_close = true;
                }
                Event::SetScale(scale) => {
                    self.window = create_window(zx_width as f32, zx_height as f32, scale);
                }
            }
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        let row = y;
        let col = x / 8;
        let bit = 7 - (x % 8);

        // Translate y (row) into ZX Spectrum memory format
        let y0 = (row & 0b00000111) << 8;
        let y1 = (row & 0b00111000) << 2;
        let y2 = (row & 0b11000000) << 5;

        let address = y2 | y1 | y0 | col;

        let bitmap = self.bitmap_buffer.buffer.lock().unwrap();
        let byte = bitmap[address];
        (byte & (1 << bit)) != 0
    }
    fn get_attribute(&self, x: usize, y: usize) -> u8 {
        let attr_x = x / 8;
        let attr_y = y / 8;
        let attr_index = (attr_y * (zx_width / 8)) + attr_x;
        self.attribute_buffer.buffer.lock().unwrap()[attr_index]
    }

    fn get_color(&self, x: usize, y: usize) -> u32 {
        let pixel = self.get_pixel(x, y);
        let attr = self.get_attribute(x, y);
        let bright = (attr & 0b1000_0000) != 0;
        let paper_index = ((attr & 0b0011_1000) >> 3) as usize;
        let ink_index = (attr & 0b0000_0111) as usize;
        if pixel {
            if bright {
                zx_bright_colors[ink_index]
            } else {
                zx_unbright_colors[ink_index]
            }
        } else {
            if bright {
                zx_bright_colors[paper_index]
            } else {
                zx_unbright_colors[paper_index]
            }
        }
    }

    pub fn run(&mut self) {
        let mut dsp_buffer: Vec<u32> = vec![0; zx_width * zx_height];
        loop {
            self.handle_events();
            if self.should_close {
                break;
            }
            {
                for (i, val) in dsp_buffer.iter_mut().enumerate() {
                    let color = self.get_color(i % zx_width, i / zx_width);
                    *val = color;
                }
            }
            self.window
                .update_with_buffer(&dsp_buffer, zx_width, zx_height)
                .expect("Could not update display");
        }
    }
}

use std::sync::{mpsc, Arc, Mutex};

use minifb::{Key, Window, WindowOptions};
use emu_lib::memory::errors::{MemoryRWCommonError, MemoryReadError, MemoryWriteError};
use emu_lib::memory::MemoryDevice;
use crate::memdsp::Event;

fn create_window(width: f32, height: f32, scale: f32) -> Window {
    Window::new(
        "Test - ESC to exit",
        (width * scale) as usize,
        (height * scale) as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window")
}

const zx_width: usize = 256;
const zx_height: usize = 192;
const zx_unbright_colors : [u32; 16] = [
    0x000000, 0x0000FF, 0x00FF00, 0x00FFFF,
    0xFF0000, 0xFF00FF, 0xFFFF00, 0xFFFFFF,
    0x000000, 0x0000FF, 0x00FF00, 0x00FFFF,
    0xFF0000, 0xFF00FF, 0xFFFF00, 0xFFFFFF,
];

const zx_bright_colors : [u32; 16] = [
    0x000000, 0x0000FF, 0x00FF00, 0x00FFFF,
    0xFF0000, 0xFF00FF, 0xFFFF00, 0xFFFFFF,
    0x000000, 0x0000FF, 0x00FF00, 0x00FFFF,
    0xFF0000, 0xFF00FF, 0xFFFF00, 0xFFFFFF,
];

#[derive(Debug,Clone)]
struct MemBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl MemBuffer {
    pub fn new(size: usize) -> MemBuffer {
        MemBuffer {
            buffer: Arc::new(Mutex::new(vec![0; size])),
        }
    }
}

impl MemoryDevice for MemBuffer {
    fn size(&self) -> usize {
        self.buffer.lock().expect("Failed to lock buffer").len()
    }

    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        let lockbuf = self.buffer.lock().expect("Failed to lock buffer");
        if addr as usize >= lockbuf.len() {
            return Err(MemoryRWCommonError::OutOfBounds(addr).into());
        }
        Ok(lockbuf[addr as usize])
    }

    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        let mut lockbuf = self.buffer.lock().expect("Failed to lock buffer");
        if addr as usize >= lockbuf.len() {
            return Err(MemoryRWCommonError::OutOfBounds(addr).into());
        }
        lockbuf[addr as usize] = data;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        self.write_8(addr, data)
    }
}

pub struct FBZXDisplay {
    window: Window,
    event_receiver: mpsc::Receiver<Event>,
    bitmap_buffer: MemBuffer,
    attribute_buffer: MemBuffer,
    should_close: bool,
}


impl FBZXDisplay {
    pub fn new(
        scale: f32,
        buffer: Arc<Mutex<Vec<u8>>>,
        event_receiver: mpsc::Receiver<Event>, 
        refresh_rate: f64
    ) -> FBZXDisplay {
        let mut window = create_window(zx_width as f32, zx_height as f32, scale);

        window.limit_update_rate(Some(std::time::Duration::from_micros((1_000_000.0 / refresh_rate) as u64)));
        FBZXDisplay {
            window,
            event_receiver,
            bitmap_buffer: MemBuffer::new(zx_width * zx_height / 8),
            attribute_buffer: MemBuffer::new(zx_width * zx_height / (8 * 8)),
            should_close: false,
        }
    }
    
    fn bmp_buffer(&self) -> Box<impl MemoryDevice>{
        Box::new(self.bitmap_buffer.clone())
    }
    
    fn attribute_buffer(&self) -> Box<impl MemoryDevice>{
        Box::new(self.attribute_buffer.clone())
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
        let byte_index = (y * zx_width + x) / 8;
        let bit_index = (y * zx_width + x) % 8;
        let byte = self.bitmap_buffer.buffer.lock().unwrap()[byte_index];
        (byte & (1 << (7 - bit_index))) != 0
    }
    fn get_attribute(&self, x: usize, y: usize) -> u8 {
        let attr_x = x / 8;
        let attr_y = y / 8;
        let attr_index = (attr_y * (zx_width / 8)) + attr_x;
        self.attribute_buffer.buffer.lock().unwrap()[attr_index]
    }
    
    fn get_color(&self, x: usize, y: usize) -> u32 {
        let pixel = self.get_pixel(x, y);
        if pixel {
            let attr = self.get_attribute(x, y);
            let bright = (attr & 0b1000_0000) != 0;
            let color_index = (attr & 0b0000_1111) as usize;
            if bright {
                zx_bright_colors[color_index]
            } else {
                zx_unbright_colors[color_index]
            }
        } else {
            0x000000
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

use std::sync::{Arc, mpsc, Mutex};

use minifb::{Key, Window, WindowOptions};

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

pub struct FBDisplay {
    width: usize,
    height: usize,
    window: Window,
    event_receiver: mpsc::Receiver<Event>,
    buffer: Arc<Mutex<Vec<u8>>>,
    should_close: bool,
}

impl FBDisplay {
    pub fn new(
        width: usize,
        height: usize,
        scale: f32,
        buffer: Arc<Mutex<Vec<u8>>>,
        event_receiver: mpsc::Receiver<Event>,
    ) -> FBDisplay {
        let mut window = create_window(width as f32, height as f32, scale);

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        FBDisplay {
            width,
            height,
            window,
            event_receiver,
            buffer,
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
                    self.window = create_window(self.width as f32, self.height as f32, scale);
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut dsp_buffer: Vec<u32> = vec![0; self.width * self.height];
        loop {
            self.handle_events();
            if self.should_close {
                break;
            }
            {
                let lockbuf = &self.buffer.lock();
                for (i, val) in dsp_buffer.iter_mut().enumerate() {
                    let data = lockbuf.as_ref().expect("Could not get reference of value")[i];
                    let r = data & 0b11100000; // first 3 bits are used for the red channel
                    let g = (data & 0b00011100) << 3; // next 3 bits are used for the green channel
                    let b = (data & 0b00000011) << 6; // last 2 bits are used for the blue channel
                    let color = u32::from_be_bytes([0, r, g, b]);
                    *val = color;
                }
            }
            self.window
                .update_with_buffer(&dsp_buffer, self.width, self.height)
                .expect("Could not update display");
        }
    }
}

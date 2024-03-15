use std::sync::{Arc, Mutex};
use std::thread;

use minifb::{Key, Window, WindowOptions};
use rand::random;

use emu_lib::memory::MemoryDevice;
use emu_lib::utils::Size;

enum Event {
    SetWidth(usize),
    Exit,
}

pub struct MemViz {
    buffer: Arc<Mutex<Vec<u8>>>,
    width: usize,
    thread: Option<thread::JoinHandle<()>>,
    events: Arc<Mutex<Vec<Event>>>,
}

fn get_height(size: usize, width: usize) -> usize {
    size.div_ceil(width)
}

impl MemViz {
    pub fn new(size: usize, width: usize) -> MemViz {
        let buffer = Arc::new(Mutex::new(vec![0; size]));
        MemViz {
            buffer,
            width,
            thread: None,
            events: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn randomize(&mut self) {
        for i in self.buffer.lock().unwrap().iter_mut() {
            *i = random();
        }
    }

    pub fn start_thread(&mut self, scale: f32) {
        let mut width = self.width;
        let mut height = self.get_height();
        let buffer = self.buffer.clone();
        let mut dsp_buffer: Vec<u32> = vec![0; width * height];
        let events = self.events.clone();
        self.thread = Some(thread::spawn(move || {
            let mut window = Window::new(
                "Test - ESC to exit",
                (width as f32 * scale) as usize,
                (height as f32 * scale) as usize,
                WindowOptions::default(),
            )
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            while window.is_open() && !window.is_key_down(Key::Escape) {
                while let Some(event) = events.lock().unwrap().pop() {
                    match event {
                        Event::SetWidth(new_width) => {
                            let size = buffer.lock().unwrap().capacity();
                            if size % new_width != 0 {
                                continue;
                            }                            width = new_width;
                            height = get_height(size, width);
                            window = Window::new(
                                "Test - ESC to exit",
                                (width as f32 * scale) as usize,
                                (height as f32 * scale) as usize,
                                WindowOptions::default(),
                            )
                                .unwrap_or_else(|e| {
                                    panic!("{}", e);
                                });
                        }
                        Event::Exit => {
                            return;
                        }
                    }
                }
                {
                    let lockbuf = buffer.lock();
                    for (i, val) in dsp_buffer.iter_mut().enumerate() {
                        let data = lockbuf.as_ref().unwrap()[i];
                        let r = data & 0b11100000;          // first 3 bits are used for the red channel
                        let g = (data & 0b00011100) << 3;   // next 3 bits are used for the green channel
                        let b = (data & 0b00000011) << 6;   // last 2 bits are used for the blue channel
                        let color = u32::from_be_bytes([
                            0,
                            r,
                            g,
                            b,
                        ]);
                        *val = color;
                    }
                }
                window.update_with_buffer(&dsp_buffer, width, height).unwrap();
            }
        }));
    }

    pub fn stop_thread(&mut self) {
        if self.thread.is_none() {
            return;
        }
        self.events.lock().unwrap().push(Event::Exit);
        self.thread.take().unwrap().join().unwrap();
    }
    pub fn get_height(&self) -> usize {
        get_height(self.buffer.lock().unwrap().len(), self.width)
    }

    pub fn set_width(&mut self, width: usize) {
        self.events.lock().unwrap().push(Event::SetWidth(width))
    }
}

impl Drop for MemViz {
    fn drop(&mut self) {
        self.stop_thread();
    }
}

impl Size for MemViz {
    fn size(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
}

impl MemoryDevice for MemViz {
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        Ok(self.buffer.lock().unwrap()[addr as usize])
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        self.buffer.lock().unwrap()[addr as usize] = data;
        Ok(())
    }
}
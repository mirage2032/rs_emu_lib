use std::sync::{Arc, Mutex};
use std::thread;

use minifb::{Key, Window, WindowOptions};
use rand::random;

use emu_lib::memory::MemoryDevice;

enum Event {
    Exit,
    SetScale(f32),
}

pub struct MemViz {
    width: usize,
    buffer: Arc<Mutex<Vec<u8>>>,
    thread: Option<thread::JoinHandle<()>>,
    events: Arc<Mutex<Vec<Event>>>,
}

fn get_height(size: usize, width: usize) -> usize {
    size.div_ceil(width)
}

fn create_window(width:f32,height:f32, scale:f32) -> Window {
    Window::new(
        "Test - ESC to exit",
        (width * scale) as usize,
        (height * scale) as usize,
        WindowOptions::default(),
    )
        .expect("Unable to create window")
}

impl MemViz {
    pub fn new(size: usize, width: usize) -> MemViz {
        let buffer = Arc::new(Mutex::new(vec![0; size]));
        MemViz {
            width,
            buffer,
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
            let mut window = create_window(width as f32, height as f32, scale);
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            while window.is_open() && !window.is_key_down(Key::Escape) {
                while let Some(event) = events.lock().unwrap().pop() {
                    match event {
                        Event::Exit => return,
                        Event::SetScale(scale) => {
                            window = create_window(width as f32, height as f32, scale);
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
        if let Some(thread) = self.thread.take() {
            self.events.lock().unwrap().push(Event::Exit);
            thread.join().unwrap();
        }
    }
    pub fn get_height(&self) -> usize {
        get_height(self.buffer.lock().unwrap().len(), self.width)
    }
    
    pub fn set_scale(&mut self, scale: f32) {
        self.events.lock().unwrap().push(Event::SetScale(scale));
    }
}

impl Drop for MemViz {
    fn drop(&mut self) {
        self.stop_thread();
    }
}

impl MemoryDevice for MemViz {
    fn size(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        Ok(self.buffer.lock().unwrap()[addr as usize])
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        self.buffer.lock().unwrap()[addr as usize] = data;
        Ok(())
    }
}
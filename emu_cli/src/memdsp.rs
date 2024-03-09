use std::sync::{Arc, Mutex};
use std::thread;

use minifb::{Key, Window, WindowOptions};
use rand::random;

use emu_lib::memory::MemDevice;

pub struct MemViz {
    buffer: Arc<Mutex<Vec<u8>>>,
    width: usize,
    should_stop: Arc<Mutex<bool>>,
    thread: Option<thread::JoinHandle<()>>,
}


impl MemViz {
    pub fn new(size: usize, width: usize) -> MemViz {
        let buffer = Arc::new(Mutex::new(vec![0; size]));
        MemViz {
            buffer,
            width,
            should_stop: Arc::new(Mutex::new(false)),
            thread: None,
        }
    }

    pub fn randomize(&mut self) {
        for i in self.buffer.lock().unwrap().iter_mut() {
            *i = random();
        }
    }

    pub fn start_thread(&mut self, scale: usize) {
        let width = self.width;
        let height = self.get_height();
        let buffer = self.buffer.clone();
        let mut dsp_buffer: Vec<u32> = vec![0; width * height];
        let should_stop = self.should_stop.clone();
        self.thread = Some(thread::spawn(move || {
            let mut window = Window::new(
                "Test - ESC to exit",
                width * scale,
                height * scale,
                WindowOptions::default(),
            )
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
            while window.is_open() && !window.is_key_down(Key::Escape) && !*should_stop.lock().unwrap() {
                for (i, val) in dsp_buffer.iter_mut().enumerate() {
                    let data = buffer.lock().unwrap()[i];
                    let color = u32::from_be_bytes([
                        0,
                        data.saturating_mul(5),
                        data,
                        data.overflowing_mul(20).0
                    ]);
                    *val = color;
                }
                window.update_with_buffer(&dsp_buffer, width, height).unwrap();
            }
        }));
    }

    pub fn stop_thread(&mut self) {
        if self.thread.is_none() {
            return;
        }
        *self.should_stop.lock().unwrap() = true;
        self.thread.take().unwrap().join().unwrap();
    }
    pub fn get_height(&self) -> usize {
        self.buffer.lock().unwrap().len().div_ceil(self.width)
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
}

impl Drop for MemViz {
    fn drop(&mut self) {
        self.stop_thread();
    }
}

impl MemDevice for MemViz {
    fn size(&self) -> usize {
        let s = self.buffer.lock().unwrap().capacity();
        return s;
    }
    fn read(&self, addr: u16) -> u8 {
        self.buffer.lock().unwrap()[addr as usize]
    }
    fn write(&mut self, addr: u16, data: u8) -> Result<(), &str> {
        self.buffer.lock().unwrap()[addr as usize] = data;
        Ok(())
    }
    fn is_read_only(&self) -> bool {
        false
    }
    fn clear(&mut self) -> Result<(), &str> {
        let mut buffer = self.buffer.lock().unwrap();
        for i in buffer.iter_mut() {
            *i = 0;
        }
        Ok(())
    }
}
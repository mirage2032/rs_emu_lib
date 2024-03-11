use std::sync::{Arc, Mutex};
use std::thread;

use minifb::{Key, Window, WindowOptions};
use rand::random;

use emu_lib::memory::MemDevice;

enum Event{
    SetWidth(usize),
    Exit
}

pub struct MemViz {
    buffer: Arc<Mutex<Vec<u8>>>,
    width: usize,
    thread: Option<thread::JoinHandle<()>>,
    events: Arc<Mutex<Vec<Event>>>
}

fn get_height(size:usize,width:usize) -> usize{
    size.div_ceil(width)
}

impl MemViz {
    pub fn new(size: usize, width: usize) -> MemViz {
        let buffer = Arc::new(Mutex::new(vec![0; size]));
        MemViz {
            buffer,
            width,
            thread: None,
            events:Arc::new(Mutex::new(vec![]))
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
                while let Some(event) = events.lock().unwrap().pop(){
                    match event {
                        Event::SetWidth(new_width) => {
                            let size = buffer.lock().unwrap().capacity();
                            if size%new_width!=0{
                                continue
                            }
                            width = new_width;
                            height = get_height(size,width);
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
                        Event::Exit=>{
                            return;
                        }
                    }
                }
                for (i, val) in dsp_buffer.iter_mut().enumerate() {
                    let data = buffer.lock().unwrap()[i];
                    let r = data&0b11100000;
                    let g = (data&0b00011100)<<3;
                    let b = (data&0b00000011)<<6;
                    let color = u32::from_be_bytes([
                        0,
                        r,
                        g,
                        b,
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
        self.events.lock().unwrap().push(Event::Exit);
        self.thread.take().unwrap().join().unwrap();
    }
    pub fn get_height(&self) -> usize {
        get_height(self.buffer.lock().unwrap().len(),self.width)
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

impl MemDevice for MemViz {
    fn size(&self) -> usize {
        self.buffer.lock().unwrap().capacity()
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
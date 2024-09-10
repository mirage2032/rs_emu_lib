use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use emu_lib::memory::MemoryDevice;
use fbdisplay::FBDisplay;
use rand::random;

mod fbdisplay;

enum Event {
    Exit,
    SetScale(f32),
}

pub struct MemViz {
    buffer: Arc<Mutex<Vec<u8>>>,
    event_sender: mpsc::Sender<Event>,
    thread: Option<thread::JoinHandle<()>>,
}

impl MemViz {
    pub fn new(size: usize, width: usize, scale: f32,refresh_rate:f64) -> MemViz {
        let buffer = Arc::new(Mutex::new(vec![0; size]));
        let buffer_clone = buffer.clone();
        let (event_sender, event_receiver) = mpsc::channel();
        let thread = Some(thread::spawn(move || {
            let mut fbdisplay = FBDisplay::new(
                width,
                size.div_ceil(width),
                scale,
                buffer_clone,
                event_receiver,
                refresh_rate,
            );
            fbdisplay.run();
        }));
        MemViz {
            buffer,
            event_sender,
            thread,
        }
    }

    pub fn randomize(&mut self) {
        let mut buffer = self.buffer.lock().expect("Failed to lock buffer");
        for i in buffer.iter_mut() {
            *i = random();
        }
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<(), &'static str> {
        self.event_sender
            .send(Event::SetScale(scale))
            .map_err(|_| "Failed to send set_scale event to thread")
    }
}

impl Drop for MemViz {
    fn drop(&mut self) {
        self.event_sender
            .send(Event::Exit)
            .expect("Could not send the exit event to thread");
        self.thread
            .take()
            .expect("Could not take the thread handle")
            .join()
            .expect("Could not join the thread");
    }
}

impl MemoryDevice for MemViz {
    fn size(&self) -> usize {
        self.buffer.lock().expect("Failed to lock buffer").len()
    }
    fn read_8(&self, addr: u16) -> Result<u8, &'static str> {
        self.buffer
            .lock()
            .or(Err("Failed to lock buffer"))?
            .get(addr as usize)
            .copied()
            .ok_or("Address out of bounds")
    }
    fn write_8(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        self.buffer
            .lock()
            .or(Err("Failed to lock buffer"))?
            .get_mut(addr as usize)
            .map(|v| *v = data)
            .ok_or("Address out of bounds")?;
        Ok(())
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), &'static str> {
        Self::write_8(self, addr, data)
    }
}

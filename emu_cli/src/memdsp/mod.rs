use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use emu_lib::memory::MemoryDevice;
use rand::random;
use emu_lib::io::InterruptType;
use emu_lib::io::iodevice::IODevice;
use emu_lib::memory::errors::{MemoryRWCommonError, MemoryReadError, MemoryWriteError};
use crate::memdsp::fbzxdisplay::{zx_inner_height, zx_inner_width, FBZXDisplay};

mod fbdisplay;
mod fbzxdisplay;

enum Event {
    Exit,
    SetScale(f32),
}

pub struct MemViz {
    bitmap_buffer: MemBuffer,
    attribute_buffer: MemBuffer,
    event_sender: mpsc::Sender<Event>,
    thread: Option<thread::JoinHandle<()>>,
    border_io: DisplayIO,
    timer_io: TimerIO
}

impl MemViz {
    pub fn new(scale: f32,refresh_rate:f64) -> MemViz {
        let (event_sender, event_receiver) = mpsc::channel();
        let bitmap_buffer = MemBuffer::new(zx_inner_width * zx_inner_height / 8);
        let bitmap_buffer_clone = bitmap_buffer.clone();
        let attribute_buffer = MemBuffer::new(zx_inner_width * zx_inner_height / (8 * 8));
        let attribute_buffer_clone = attribute_buffer.clone();
        let border_io = DisplayIO{val:Arc::new(Mutex::new(0))};
        let border_io_clone = border_io.clone();
        let timer_io = TimerIO{should_interrupt:Arc::new(Mutex::new(false))};
        let timer_io_clone = timer_io.clone();
        let thread = Some(thread::spawn(move || {
            let mut fbdisplay = FBZXDisplay::new(
                bitmap_buffer_clone,
                attribute_buffer_clone,
                border_io_clone,
                timer_io_clone,
                scale,
                event_receiver,
                refresh_rate
            );
            fbdisplay.run();
        }));
        MemViz {
            bitmap_buffer,
            attribute_buffer,
            event_sender,
            thread,
            border_io,
            timer_io

        }
    }
    pub fn bmp_buffer(&self) -> Box<impl MemoryDevice>{
        Box::new(self.bitmap_buffer.clone())
    }

    pub fn attribute_buffer(&self) -> Box<impl MemoryDevice>{
        Box::new(self.attribute_buffer.clone())
    }

    pub fn border_io(&self) -> Box<impl IODevice> {
        Box::new(self.border_io.clone())
    }
    
    pub fn timer_io(&self) -> Box<impl IODevice> {
        Box::new(self.timer_io.clone())
    }

    pub fn randomize(&mut self) {
        let mut bmp_buff = self.bitmap_buffer.buffer.lock().expect("Could not lock buffer");
        for v in bmp_buff.iter_mut() {
            *v = random();
        }
        let mut attribute_buff = self.attribute_buffer.buffer.lock().expect("Could not lock buffer");
        for v in attribute_buff.iter_mut() {
            *v = random()
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

#[derive(Debug, Clone)]
pub struct DisplayIO{
    val: Arc<Mutex<u8>>,
}

impl IODevice for DisplayIO {
    fn ports(&self) -> Vec<u8> {
        vec![0xFE]
    }
    fn read(&self, _port: u8) -> Result<u8, &'static str> {
        Ok(*self.val.lock().map_err(|_| "Could not acquire lock")?)
    }

    fn write(&mut self, _pin: u8, data: u8) -> Result<(), &'static str> {
        *self.val.lock().map_err(|_| "Could not acquire lock")? = data;
        Ok(())
    }

    fn step(&mut self) {

    }

    fn will_interrupt(&self) -> Option<InterruptType> {
        None
    }

    fn ack_int(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TimerIO{
    should_interrupt: Arc<Mutex<bool>>,
}

impl TimerIO{
    pub fn interrupt(&self){
        *self.should_interrupt.lock().expect("Could not lock interrupt") = true;
    }
}

impl IODevice for TimerIO {
    fn ports(&self) -> Vec<u8> {
        vec![]
    }
    fn read(&self, _port: u8) -> Result<u8, &'static str> {
        Err("Cannot read timer")
    }

    fn write(&mut self, _: u8, _: u8) -> Result<(), &'static str> {
        Err("Cannot write timer")
    }

    fn step(&mut self) {
    }

    fn will_interrupt(&self) -> Option<InterruptType> {
        if *self.should_interrupt.lock().expect("Could not lock interrupt") {
            Some(InterruptType::IM1)
        } else {
            None
        }
    }

    fn ack_int(&mut self) -> Result<(), &'static str> {
        *self.should_interrupt.lock().map_err(|_| "Could not acquire lock")? = false;
        Ok(())
    }
}
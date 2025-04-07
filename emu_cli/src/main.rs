use emu_lib::memory::MemoryDevice;
use std::path::PathBuf;
use emu_lib::cpu::Cpu;

use emu_lib::cpu::{
    registers::{AllRegisters, GPByteRegisters},
};
use emu_lib::cpu::instruction::ExecutableInstruction;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::io::IO;
use emu_lib::io::iodevice::IORegister;
use emu_lib::memory::{memdevices::RAM, Memory};

mod memdsp;
use memdsp::MemViz;

fn print_registers(registers: &AllRegisters) {
    println!("PC: {:04X}, SP: {:04X}", registers.pc, registers.sp);
    fn print_gp(gp: &GPByteRegisters, suffix: &str) {
        println!(
            "AF{suffix}: {:04X}, BC{suffix}: {:04X}, DE{suffix}: {:04X}, HL{suffix}: {:04X}",
            gp.af, gp.bc, gp.de, gp.hl
        );
    }
    for (i, gp_regs) in registers.gp.iter().enumerate() {
        print_gp(gp_regs, &String::from("'").repeat(i));
    }
    for (key, value) in &registers.other8bit {
        print!("{}: {}, ", key.to_uppercase(), value);
    }
    for (key, value) in &registers.other16bit {
        print!("{}: {}, ", key.to_uppercase(), value);
    }
    println!();
}

fn main() {
    let refresh_rate = 50.08;
    let dsp = MemViz::new(2.0, refresh_rate);
    let bitmap_mem = dsp.bmp_buffer();
    let bitmap_len = bitmap_mem.size();
    let attribute_mem = dsp.attribute_buffer();
    let attribute_len = attribute_mem.size();
    let border_io = dsp.border_io();
    let timer_io = dsp.timer_io();
    // dsp.randomize();
    // thread::sleep(Duration::from_secs(2));
    println!("Creating emulator");
    let mut memory = Memory::new();
    memory.add_device(Box::new(RAM::new(0x4000)));
    memory.add_device(bitmap_mem);
    memory.add_device(attribute_mem);
    memory.add_device(Box::new(RAM::new(0x10000-bitmap_len-attribute_len-0x4000)));
    let mut emulator: Emulator<Z80> = Emulator::new_w_mem(memory);
    let mut io = IO::new();
    //vec with all 0..FF except 0xFE
    let mut other_io = vec![];
    for v in 0..0x100 {
        if v!=0xFE {
            other_io.push(v as u8);
        }
    }
    io.add_device(Box::new(IORegister::new(other_io))).unwrap();
    io.add_device(timer_io).expect("Failed to add device");
    io.add_device(border_io).expect("Failed to add device");
    emulator.io = io;
    let rom_path: PathBuf = PathBuf::from("roms/zx48.rom");
    // let z80_file = include_bytes!("../roms/f.z80");
    //a .z80 file, byte 5 and 6 of the file store the pc
    // let pc = u16::from_le_bytes([z80_file[5],z80_file[6]]);
    // let sp =  u16::from_le_bytes([z80_file[6],z80_file[7]]);
    // emulator.cpu.registers.pc = pc;
    // println!("{:04X}",pc);
    // emulator.cpu.registers.sp = sp;
    // let rom = &z80_file[87..];
    // println!("Loading rom: {}", rom_path.to_str().unwrap());
    match emulator.memory.load_file(&rom_path,true){
        Ok(_) => {}
        Err(e) => {
            panic!("Error loading rom: {:?}", e);
        }
    };
    println!("Running emulator");
    // print_registers(emulator.cpu.registers());
    let freq = 3_500_000.0;
    let stop_reason = emulator.run_with_callback(
        freq,
        Some(move |emu: &mut Emulator<_>, instruction: &dyn ExecutableInstruction<_>| {
            // println!("{}", instruction);
            // println!("{:?}",emu.io.read(0xFE));
            //
            // print_registers(emu.cpu.registers);
        }),
        // 1
        freq as f64 / refresh_rate,
    );
    println!("Emulator stopped");
    match stop_reason {
        emu_lib::emulator::StopReason::Breakpoint => println!("Breakpoint"),
        emu_lib::emulator::StopReason::Halt => println!("Halted"),
        emu_lib::emulator::StopReason::Error(e) => {
            let pc = emulator.cpu.registers().pc;
            let instruction = emulator
                .cpu
                .parser()
                .ins_from_machinecode(&emulator.memory, *pc)
                .expect("Error decoding instruction");
            println!("Error: {} while executing \"{}\"", e, instruction)
        }
    }
}

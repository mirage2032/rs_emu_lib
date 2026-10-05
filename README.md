# Emulation and debugging library for Z80
![build status](https://github.com/mirage2032/rs_emu_lib/actions/workflows/rust.yml/badge.svg)

## Status: WIP

## Compatibility
The project is written in Rust and should be compatible with any platform that supports Rust.

## Description
The project is a library that provides emulation and debugging capabilities for the Z80 CPU.  
The library is designed to be used in other projects that require Z80 emulation and debugging.
The IO and Memory can be easily extended to support custom devices and memory configurations.

## Interrupts
The CPU keeps its interrupt state in `emu.cpu.interrupts`: IFF1, IFF2, the mode set by `IM 0/1/2`,
and the requests waiting to be accepted. Save states carry it, and a reset (`Z80::default()`) clears it.
To raise an interrupt:

```rust
emu.request_int(0xFF);        // INT, held until the CPU accepts it. 0xFF is the byte the device
                              // puts on the data bus: RST 38h in IM 0, the vector's low byte in IM 2.
emu.set_int_line(true, 0xFF); // hold INT active (level-triggered) until set_int_line(false, _)
emu.request_nmi();            // NMI, taken before the next instruction

let handle = emu.interrupt_handle(); // Clone + Send: raise interrupts from another thread
std::thread::spawn(move || handle.int(0xFF));
```

A device that drives INT itself implements `IODevice::int_pending` and `IODevice::int_ack`.

Accepting an interrupt is a `step()` of its own, which returns a pseudo-instruction such as
`INT IM1 -> 0x0038` with the right T-states: 11 for NMI, 13 for IM 0 and IM 1, and 19 for IM 2.
HALT is a state: a halted CPU idles in 4 T-state steps until an interrupt arrives. `run_ticks` stops
with `StopReason::Halt` only when nothing but an NMI could wake the CPU (halted with interrupts disabled).

## This project will serve as my Computer Science Bachelor's project.


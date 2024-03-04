mod memory;
mod cpu;

use pyo3::prelude::*;

use cpu::*;
use memory::*;
use emu_lib::emulator::Emulator;


#[pyclass(name = "Emulator")]
struct PyEmulator {
    emulator: Emulator,
}

#[pymethods]
impl PyEmulator {
    #[new]
    fn new(cputype: PyCPUType) -> Self {
        PyEmulator { emulator: Emulator::new(cputype.into()) }
    }

    #[setter]
    fn set_cpu_type(&mut self, cputype: PyCPUType) {
        self.emulator.set_cpu_type(cputype.into());
    }
    #[getter]
    fn get_cpu_type(&self) -> PyCPUType {
        self.emulator.cpu.type_of().into()
    }

    fn step(&mut self) -> PyResult<()> {
        match self.emulator.step() {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(e)),
        }
    }
}

#[pymodule]
fn py_emu_lib(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    register_cpu_module(py, m)?;
    register_memory_module(py, m)?;
    m.add_class::<PyEmulator>()?;
    Ok(())
}
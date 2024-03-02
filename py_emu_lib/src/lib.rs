use pyo3::prelude::*;

use emu_lib::cpu::CPUType;
use emu_lib::emulator::Emulator;

#[pyclass(name = "Emulator")]
pub struct PyEmulator {
    emulator: Emulator,
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct PyCPUType {
    cpu_type: CPUType,
}

#[pymethods]
impl PyEmulator {
    #[new]
    fn new(cputype: PyCPUType) -> Self {
        PyEmulator { emulator: Emulator::new(cputype.cpu_type) }
    }

    #[setter]
    fn set_cpu_type(&mut self, cputype: PyCPUType) {
        self.emulator.set_cpu_type(cputype.cpu_type);
    }
    #[getter]
    fn get_cpu_type(&self) -> PyCPUType {
        PyCPUType { cpu_type: self.emulator.cpu.type_of() }
    }

    fn step(&mut self) -> PyResult<()> {
        match self.emulator.step() {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(e)),
        }
    }
}

#[pymodule]
fn py_emu_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyEmulator>()?;
    Ok(())
}
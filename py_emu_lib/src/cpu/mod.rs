use pyo3::prelude::*;

use emu_lib::cpu::*;

#[pyclass(name = "CPUType")]
#[derive(Clone, Copy)]
pub enum PyCPUType {
    Z80,
    I8080,
}

impl From<PyCPUType> for CPUType {
    fn from(pycputype: PyCPUType) -> Self {
        match pycputype {
            PyCPUType::Z80 => CPUType::Z80,
            PyCPUType::I8080 => CPUType::I8080,
        }
    }
}

impl From<CPUType> for PyCPUType {
    fn from(cputype: CPUType) -> Self {
        match cputype {
            CPUType::Z80 => PyCPUType::Z80,
            CPUType::I8080 => PyCPUType::I8080,
        }
    }
}

pub fn register_cpu_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "cpu")?;
    submodule.add_class::<PyCPUType>()?;
    parent_module.add_submodule(submodule)?;
    Ok(())
}
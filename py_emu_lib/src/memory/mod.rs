use pyo3::*;
use pyo3::prelude::*;

use memdevice::*;

mod memdevice;
use emu_lib::memory::*;

#[pyclass(name = "Memory")]
struct PyMemory {
    memory: Memory,
}


#[pymethods]
impl PyMemory {
    #[new]
    #[pyo3(signature = (*_args, **_kwargs))]
    fn new(_args: &PyAny, _kwargs: Option<&PyAny>) -> Self {
        PyMemory { memory: emu_lib::memory::Memory::new() }
    }
    #[staticmethod]
    fn empty() -> Self {
        PyMemory { memory: emu_lib::memory::Memory::default() }
    }
    fn write8(&mut self, addr: u16, data: u8) -> PyResult<()> {
        match self.memory.write_8(addr, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e)),
        }
    }

    fn write16(&mut self, addr: u16, data: u16) -> PyResult<()> {
        match self.memory.write_16(addr, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e)),
        }
    }

    fn read8(&self, addr: u16) -> PyResult<u8> {
        match self.memory.read_8(addr) {
            Ok(v) => Ok(v),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e)),
        }
    }

    fn read16(&self, addr: u16) -> PyResult<u16> {
        match self.memory.read_16(addr) {
            Ok(v) => Ok(v),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e)),
        }
    }
    
    fn len(&self) -> usize {
        self.memory.len()
    }
    
    fn save(&self, filename: String) -> PyResult<()> {
        match &self.memory.save(&*filename) {
            Ok(_) => Ok(()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }
    fn load(&mut self, filename: String) -> PyResult<()> {
        match self.memory.load(&*filename) {
            Err(vec) => {
                for e in vec {
                    match e {
                        MemoryError::FileError => return Err(pyo3::exceptions::PyException::new_err("Open file error")),
                        MemoryError::ReadError => return Err(pyo3::exceptions::PyException::new_err("Read error")),
                        _ => {}
                    }
                }
                return Ok(());
            }
            _ => Ok(()),
        }
    }

    fn clear(&mut self) {
        self.memory.clear();
    }
    pub fn add_device(&mut self, device: PyMemDevice) {
        self.memory.add_device(Box::new(device));
    }
}

pub fn register_memory_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "memory")?;
    submodule.add_class::<PyMemory>()?;
    submodule.add_class::<PyMemDevice>()?;
    parent_module.add_submodule(submodule)?;
    Ok(())
}

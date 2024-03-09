use pyo3::prelude::*;
use pyo3::types::PyTuple;

use emu_lib::memory::MemDevice;

#[pyclass(subclass, name = "MemDevice")]
#[derive(Clone)]
pub struct PyMemDevice {
    size: Option<PyObject>,
    read: Option<PyObject>,
    write: Option<PyObject>,
    ro: Option<PyObject>,
    clear: Option<PyObject>,

}

impl ToPyObject for PyMemDevice {
    fn to_object(&self, py: Python) -> PyObject {
        // You can use PyCell to wrap the Rust struct into a Python object
        PyCell::new(py, Self {
            size: Some(self.size.as_ref().unwrap().clone_ref(py)),
            read: Some(self.read.as_ref().unwrap().clone_ref(py)),
            write: Some(self.write.as_ref().unwrap().clone_ref(py)),
            ro: Some(self.ro.as_ref().unwrap().clone_ref(py)),
            clear: Some(self.clear.as_ref().unwrap().clone_ref(py)),
        })
            .expect("Failed to create PyCell")
            .into()
    }
}

#[pymethods]
impl PyMemDevice {
    #[new]
    fn new(_args: &PyAny, _kwargs: Option<&PyAny>) -> Self {
        PyMemDevice {
            size: None,
            read: None,
            write: None,
            ro: None,
            clear: None,
        }
    }
    fn size(&self) -> usize {
        (self as &dyn MemDevice).size()
    }
    fn set_cb_size(&mut self, size: PyObject) {
        self.size = Some(size);
    }

    fn read(&self, addr: u16) -> u8 {
        (self as &dyn MemDevice).read(addr)
    }

    fn set_cb_read(&mut self, read: PyObject) {
        self.read = Some(read);
    }

    fn write(&mut self, addr: u16, data: u8) -> PyResult<()> {
        let res = (self as &mut dyn MemDevice).write(addr, data);
        res.map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))
    }

    fn set_cb_write(&mut self, write: PyObject) {
        self.write = Some(write);
    }
    
    fn is_read_only(&self) -> bool {
        (self as &dyn MemDevice).is_read_only()
    }
    
    fn set_cb_ro(&mut self, ro: PyObject) {
        self.ro = Some(ro);
    }
    
    fn clear(&mut self) -> PyResult<()> {
        let res = (self as &mut dyn MemDevice).clear();
        res.map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))
    }
    
    fn set_cb_clear(&mut self, clear: PyObject) {
        self.clear = Some(clear);
    }
}

impl MemDevice for PyMemDevice {
    fn size(&self) -> usize {
        let mut result: usize = 0;
        let _ = Python::with_gil(|py| {
            result = self.size.as_ref().unwrap().call0(py).unwrap().extract::<usize>(py).unwrap();
        });
        return result;
    }
    fn read(&self, addr: u16) -> u8 {
        let mut result: u8 = 0;
        let _ = Python::with_gil(|py| {
            let args = PyTuple::new(py, &[addr]);
            result = self.read.as_ref().unwrap().call1(py, args).unwrap().extract::<u8>(py).unwrap();
        });
        return result;
    }
    fn write(&mut self, addr: u16, data: u8) -> Result<(), &str> {
        let _ = Python::with_gil(|py| {
            let args = PyTuple::new(py, &[addr, data as u16]);
            self.write.as_ref().unwrap().call1(py, args).unwrap();
        });
        Ok(())
    }
    fn is_read_only(&self) -> bool {
        let mut result: bool = false;
        let _ = Python::with_gil(|py| {
            result = self.ro.as_ref().unwrap().call0(py).unwrap().extract::<bool>(py).unwrap();
        });
        return result;
    }
    fn clear(&mut self) -> Result<(), &str> {
        let _ = Python::with_gil(|py| {
            self.clear.as_ref().unwrap().call0(py).unwrap();
        });
        Ok(())
    }
}
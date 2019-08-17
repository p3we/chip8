extern crate itertools;
extern crate rand;
extern crate pyo3;

mod isa;
mod machine;

use std::os::raw::{c_void, c_int};

use pyo3::prelude::*;
use pyo3::PySequenceProtocol;
use pyo3::PyIterProtocol;
use pyo3::PyBufferProtocol;
use pyo3::exceptions::PyValueError;
use pyo3::ffi;
use pyo3::AsPyPointer;
use machine::Machine;


/// A python Chip8 emulator implemented in Rust
#[pyclass(name="Machine", module="chip8")]
struct PyMachine {
    m: Machine
}

impl PyMachine {
    fn get_rom(self: &PyMachine) -> &[u8] {
        &self.m.mem().rom[..]
    }

    fn get_ram(self: &PyMachine) -> &[u8] {
        &self.m.mem().ram[..]
    }

    fn get_fb(self: &PyMachine) -> &[u8] {
        &self.m.mem().fb[..]
    }
}

#[pymethods]
impl PyMachine {
    #[new]
    fn new() -> Self {
        Self {
            m: Machine::new()
        }
    }

    fn reg(&self, i: usize) -> u8 {
        self.m.cpu().r[i]
    }

    #[getter]
    fn i(&self) -> usize {
        self.m.cpu().i
    }

    #[getter]
    fn pc(&self) -> usize {
        self.m.cpu().pc
    }

    #[getter]
    fn sp(&self) -> usize {
        self.m.cpu().sp
    }

    #[getter]
    fn dt(&self) -> u8 {
        self.m.cpu().dt
    }

    #[getter]
    fn st(&self) -> u8 {
        self.m.cpu().st
    }

    #[getter]
    fn rom(py_self: Py<PyMachine>, py: Python) -> PyResult<Py<PyMemoryView>> {
        Py::new(py, PyMemoryView {owner: py_self.clone(), getter: PyMachine::get_rom})
    }

    #[getter]
    fn ram(py_self: Py<PyMachine>, py: Python) -> PyResult<Py<PyMemoryView>> {
        Py::new(py, PyMemoryView {owner: py_self.clone(), getter: PyMachine::get_ram})
    }

    #[getter]
    fn framebuffer(py_self: Py<PyMachine>, py: Python) -> PyResult<Py<PyMemoryView>> {
        Py::new(py, PyMemoryView {owner: py_self.clone(), getter: PyMachine::get_fb})
    }
}

#[pymethods]
impl PyMachine {
    fn load(&mut self, filename: &str) -> PyResult<usize> {
        match self.m.load(filename) {
            Ok(value) => Ok(value),
            Err(err) => Err(PyErr::from(err))
        }
    }

    fn reset(&mut self) {
        self.m.reset()
    }

    fn step<'p>(&mut self, py: Python<'p>) -> PyResult<PyObject> {
        match self.m.step() {
            Some(value) => Ok((value.0.into_py(py), value.1.to_string().into_py(py)).into_py(py)),
            None => Err(PyValueError::new_err("invalid opcode"))
        }
    }

    fn tick(&mut self) {
        self.m.tick()
    }

    fn keyevent(&mut self, key: usize, state: bool) {
        self.m.keys[key] = state;
    }
}

// A python bytes like object implemented in Rust
#[pyclass(name="MachineMemoryView", module="chip8")]
struct PyMemoryView {
    owner: Py<PyMachine>,
    getter: fn(&PyMachine) -> &[u8]
}

impl PyMemoryView {
    fn at(&self, py: Python, index: usize) -> PyResult<u8> {
        let machine: PyRef<PyMachine> = self.owner.try_borrow(py)?;
        let mem = (self.getter)(&*machine);
        if index < mem.len() {
            Ok(mem[index])
        }
        else {
            Err(PyValueError::new_err("index out of range"))
        }
    }
}

#[pyproto]
impl PyBufferProtocol for PyMemoryView {
    fn bf_getbuffer(slf: PyRefMut<Self>, view: *mut ffi::Py_buffer, flags: c_int) -> PyResult<()> {
        let machine: PyRef<PyMachine> = slf.owner.try_borrow(slf.py())?;
        let mem: &[u8] = (slf.getter)(&machine);
        let obj = slf.as_ptr();
        let buff = mem.as_ptr() as *mut c_void;
        let size = mem.len() as isize;
        unsafe {
            ffi::PyBuffer_FillInfo(view, obj, buff, size, 1, flags)
        };
        Ok(())
    }

    fn bf_releasebuffer(_slf: PyRefMut<Self>, _view: *mut ffi::Py_buffer) -> PyResult<()> {
        Ok(())
    }
}

#[pyproto]
impl PySequenceProtocol for PyMemoryView {
    fn __getitem__(&self, index: isize) -> PyResult<u8> {
        let gil = Python::acquire_gil();
        self.at(gil.python(), index as usize)
    }

    fn __len__(&self) -> PyResult<usize> {
        let gil = Python::acquire_gil();
        let machine: PyRef<PyMachine> = self.owner.try_borrow(gil.python())?;
        let mem: &[u8] = (self.getter)(&machine);
        Ok(mem.len())
    }
}

#[pyproto]
impl PyIterProtocol for PyMemoryView {
    fn __iter__(slf: Py<Self>) -> PyResult<Py<PyMemoryViewIter>> {
        let gil = Python::acquire_gil();
        let iter = PyMemoryViewIter {owner: slf.clone(), index: 0};
        Py::new(gil.python(), iter)
    }
}

// A Python iterator implementaed in Rust
#[pyclass(name="MachineMemoryViewIter", module="chip8")]
struct PyMemoryViewIter {
    owner: Py<PyMemoryView>,
    index: usize
}

#[pyproto]
impl PyIterProtocol for PyMemoryViewIter {
    fn __iter__(slf: Py<Self>) -> PyResult<Py<PyMemoryViewIter>> {
        Ok(slf)
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<u8> {
        let py = slf.py();
        let result = slf.owner.borrow(py).at(py, slf.index);
        slf.index += 1;
        match result {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }
}

/// A Python module implemented in Rust
#[pymodule]
fn chip8(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyMachine>()?;
    Ok(())
}

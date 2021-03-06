#![feature(vec_remove_item)]

#[macro_use]
extern crate cpython;
#[macro_use]
extern crate lazy_static;
extern crate brotli as _brotli;


use std::cell;
use std::hash::{Hasher, BuildHasher};
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::io::Read;

use cpython::{PyObject, PyResult, Python, PyTuple, PyDict, PyErr, NoArgs};
use cpython::exc::{ValueError, IndexError};
use _brotli::enc::reader::CompressorReader;


py_module_initializer!(pycontw, initpycontw, PyInit_pycontw, |py, m| {
    m.add(py, "__doc__", "PyCon TW 2017")?;
    m.add(py, "hello",              py_fn!(py, hello(name: &str)))?;
    m.add(py, "print",              py_fn!(py, print(*args, **kwargs)))?;
    m.add(py, "simple_hash",        py_fn!(py, simple_hash(data: u64)))?;
    m.add(py, "simple_random_hash", py_fn!(py, simple_random_hash(data: u64)))?;
    m.add(py, "brotli",             py_fn!(py, brotli(data: Vec<u8>)))?;
    m.add_class::<Vector>(py)?;
    Ok(())
});

////////////////////////////////////////
// hello function
////////////////////////////////////////

fn hello(_: Python, name: &str) -> PyResult<String> {
    Ok(format!("It's PyCon TW 2017, hello {} !!!", name))
}

////////////////////////////////////////
// print function
////////////////////////////////////////

fn print(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    for arg in args.iter(py) {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.items(py) {
            println!("{} = {}", key, val);
        }
    }
    Ok(py.None())
}

////////////////////////////////////////
// hash function
////////////////////////////////////////

fn simple_hash(_: Python, data: u64) -> PyResult<u64> {
    let mut s = DefaultHasher::new();
    s.write_u64(data);
    Ok(s.finish())
}


////////////////////
// With Random State
////////////////////

lazy_static! {
    static ref STATE: RandomState = RandomState::new();
}

fn simple_random_hash(_: Python, data: u64) -> PyResult<u64> {
    let mut s = STATE.build_hasher();
    s.write_u64(data);
    Ok(s.finish())
}

////////////////////////////////////////
// brotli
////////////////////////////////////////

fn brotli(_: Python, data: Vec<u8>) -> PyResult<Vec<u8>> {
    let quality = 5_u32;
    let lg_window_size = 20_u32;
    let mut enc_data = vec![];
    let mut reader = CompressorReader::new(data.as_slice(),
                                           4096,   // buffer size
                                           quality,
                                           lg_window_size);
    let _ = reader.read_to_end(&mut enc_data);
    Ok(enc_data)
}

////////////////////////////////////////
// Vector class
////////////////////////////////////////

py_class!(class Vector |py| {
    data vec: cell::RefCell<Vec<u8>>;

    def __new__(_cls) -> PyResult<Vector> {
        Vector::create_instance(
            py,
            cell::RefCell::new(Vec::new()),
        )
    }

    def __str__(&self) -> PyResult<String> {
        return Ok(format!("{:?}", self.vec(py).borrow()));
    }

    def __repr__(&self) -> PyResult<String> {
        return Ok(format!("{:?}", self.vec(py).borrow()));
    }

    def __len__(&self) -> PyResult<usize> {
        Ok(self.vec(py).borrow().len())
    }

    def append(&self, data: u8) -> PyResult<PyObject> {
        self.vec(py).borrow_mut().push(data);
        Ok(py.None())
    }

    def extend(&self, other: Vector) -> PyResult<PyObject> {
        // use clone to keep the old one
        self.vec(py).borrow_mut().append(&mut other.vec(py).borrow().clone());
        Ok(py.None())
    }

    def clear(&self) -> PyResult<PyObject> {
        self.vec(py).borrow_mut().clear();
        Ok(py.None())
    }

    def sort(&self) -> PyResult<PyObject> {
        self.vec(py).borrow_mut().sort();
        Ok(py.None())
    }

    def reverse(&self) -> PyResult<PyObject> {
        self.vec(py).borrow_mut().reverse();
        Ok(py.None())
    }

    def copy(&self) -> PyResult<Vector> {
        Ok(
            Vector::create_instance(
                py,
                cell::RefCell::new(self.vec(py).borrow().clone()),
            )?
        )
    }

    def remove(&self, data: u8) -> PyResult<PyObject> {
        // nightly now
        self.vec(py)
            .borrow_mut()
            .remove_item(&data)
            .map_or(Err(PyErr::new::<ValueError, NoArgs>(py, NoArgs)),  // None -> ValueError
                    |_| Ok(py.None()))  // T -> None
    }

    def insert(&self, index: isize, data: u8) -> PyResult<PyObject> {
        let mut vec = self.vec(py).borrow_mut();
        let length = vec.len();
        let index = match index {
            x if x < 0 => 0,
            x if x as usize > length => length,
            _ => index as usize,
        };
        vec.insert(index, data);
        Ok(py.None())
    }

    def count(&self, data: u8) -> PyResult<usize> {
        Ok(self.vec(py).borrow().iter().filter(|&v| *v == data).count())
    }

    def index(&self, data: u8) -> PyResult<usize> {
        // TODO: start, stop
        self.vec(py)
            .borrow()
            .iter()
            .position(|&v| v == data)
            .map_or(Err(PyErr::new::<ValueError, NoArgs>(py, NoArgs)),
                    |v| Ok(v))
    }

    def pop(&self, index: isize) -> PyResult<u8> {
        // TODO: default index to 0
        let index = {
            let vec = self.vec(py).borrow();
            let length = vec.len();
            match index {
                x if x < 0 => (length as isize + x) as usize,
                x if x as usize > length => return Err(PyErr::new::<IndexError, NoArgs>(py, NoArgs)),
                _ => index as usize,
            }
        };
        Ok(self.vec(py).borrow_mut().remove(index))
    }

    // TODO:
    // * assignment
    // * sort by key
    // * sort with reverse
    // * __iter__
});

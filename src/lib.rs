#[macro_use]
extern crate cpython;
#[macro_use]
extern crate lazy_static;
extern crate brotli as _brotli;


use std::hash::{Hasher, BuildHasher};
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::io::Read;

use cpython::{PyObject, PyResult, Python, PyTuple, PyDict};
use _brotli::enc::reader::CompressorReader;


py_module_initializer!(pycontw, initpycontw, PyInit_pycontw, |py, m| {
    m.add(py, "__doc__", "PyCon TW 2017")?;
    m.add(py, "hello",              py_fn!(py, hello(name: &str)))?;
    m.add(py, "print",              py_fn!(py, print(*args, **kwargs)))?;
    m.add(py, "simple_hash",        py_fn!(py, simple_hash(data: u64)))?;
    m.add(py, "simple_random_hash", py_fn!(py, simple_random_hash(data: u64)))?;
    m.add(py, "brotli",             py_fn!(py, brotli(data: Vec<u8>)))?;
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

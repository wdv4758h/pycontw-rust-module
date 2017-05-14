#[macro_use]
extern crate cpython;

use cpython::{PyObject, PyResult, Python, PyTuple, PyDict};


py_module_initializer!(pycontw, initpycontw, PyInit_pycontw, |py, m| {
    m.add(py, "__doc__", "PyCon TW 2017")?;
    m.add(py, "hello", py_fn!(py, hello(name: &str)))?;
    m.add(py, "print", py_fn!(py, print(*args, **kwargs)))?;
    Ok(())
});

fn hello(_: Python, name: &str) -> PyResult<String> {
    Ok(format!("It's PyCon TW 2017, hello {} !!!", name))
}

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

========================================
PyCon TW - Python Module in Rust
========================================


.. contents:: Table of Contents


Installation
========================================

Install Rust compiler:

.. code-block:: sh

    curl https://sh.rustup.rs
    sh rustup-init.sh


Install ``setuptools-rust``:

.. code-block:: sh

    pip install setuptools-rust


Build Project:

.. code-block:: sh

    python setup.py install



Project Skeleton
========================================

Rust part:

* Cargo.toml
* src/


Python part:

* setup.py
* pycontw/



Special Thanks
========================================

* `rust-cpython <https://github.com/dgrunwald/rust-cpython>`_ for writing CPython extension in Rust
* `setuptools-rust <https://github.com/fafhrd91/setuptools-rust>`_ for wrapping Rust code for Python setuptools

#!/usr/bin/env python3

from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ModuleNotFoundError:
    print("Please install 'setuptools_rust' first")
    import sys
    sys.exit(1)

setup(
    name='pycontw',
    version='2017',
    rust_extensions=[RustExtension('pycontw', './Cargo.toml')],
    packages=['pycontw'],
    # Rust extensions are not zip safe, just like C extensions
    zip_safe=False,
)

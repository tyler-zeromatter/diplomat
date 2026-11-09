from setuptools import Extension, setup
from glob import glob

import os

libraries = ["diplomat_example"]

if os.name == 'nt':
    libraries += ["kernel32", "advapi32", "ntdll", "userenv", "Ws2_32", "dbghelp"]

setup(
    ext_modules=[
        Extension(
            name="icu4x",
            sources=glob("src/*.c"),
            include_dirs=["src/include"],
            libraries=libraries,
            library_dirs=["../../target/debug"],
            py_limited_api=True
        ),
    ]
)
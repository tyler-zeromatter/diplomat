from setuptools import Extension, setup
from glob import glob

setup(
    ext_modules=[
        Extension(
            name="_somelib",
            sources=glob("src/*.c"),
            include_dirs=["src/include"],
            py_limited_api=True
        ),
    ]
)
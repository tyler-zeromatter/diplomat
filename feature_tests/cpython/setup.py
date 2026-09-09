from setuptools import Extension, setup
from glob import glob

setup(
    ext_modules=[
        Extension(
            name="_somelib",
            sources=glob("src/*.c")
        ),
    ]
)
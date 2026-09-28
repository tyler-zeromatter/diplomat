# CPython feature tests

## Building with Debug Symbols

### Debug Symbols for `somelib` only
This is a WIP, currently I've just been building with ALL debug symbols (including CPython)

### CPython Debug Builds
You'll need to build a debug build of CPython: https://github.com/python/cpython

Then, make a `.venv` with `python_d`:
```bash
python_d -m venv .venv
```

Install setuptools:
```bash
pip install setuptools
```

Finally, once you've generated the source files (with `cargo make gen-cpython-features`), you can build with pip:

```bash
pip install . -C--build-option=build_ext -C--build-option=--debug -C--build-option=-L"/path/to/cpython/[Where python_d lib is located]"
```

I'm on Windows running x86_64, and I've placed CPython next to the Diplomat repo, so for me this invocation is:
```bash
pip install . -C--build-option=build_ext -C--build-option=--debug -C--build-option=-L"../../../cpython/PCbuild/amd64"
```
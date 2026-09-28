# CPython Backend

## Minimum Python Version
Currently, the minimum version of Python we support is `>= 3.10.11`. This is because it is (or was, at the time of development) just barely inside of the security window of Python support: https://devguide.python.org/versions/

There are also helper functions (like `PyModule_AddObjectRef`) in Python 3.10 that the backend relies on.

## Limited C API
We build with the limited C API in mind: https://docs.python.org/3/c-api/stable.html#limited-c-api

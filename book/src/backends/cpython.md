# CPython Backend

## Minimum Python Version
Currently, the minimum version of Python we support is `>= 3.12`. This is because it currently relies on https://docs.python.org/3/c-api/object.html#c.PyObject_GetTypeData, so that everything can be run on the Stable ABI.

## Limited C API
We build with the limited C API in mind: https://docs.python.org/3/c-api/stable.html#limited-c-api

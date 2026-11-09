#include "diplomat_runtime_common.h"

inline PyObject* Diplomat_bool_to_py(bool b) {
    if (b) {
        return Py_True;
    } else {
        return Py_False;
    }
}

inline PyObject* Diplomat_long_to_py(long l) {
    PyObject* out = PyLong_FromLong(l);
    return out;
}
#include "include/DefaultEnum.h"

#include "diplomat_runtime_common.h"

int DefaultEnum_init(PyObject* module) {
    // Python enums can be passed without us needing to hold a reference, so we just create straight from the `Enum` class:
    auto enum_args = PyTuple_Pack(1, PyUnicode_FromString("DefaultEnum"));
    if (enum_args == NULL) {
        return -1;
    }

    auto enum_def = PyObject_Call(diplomat_internals.enum_ty, enum_args, Py_None);
    Py_DECREF(enum_args);
    if (enum_def == NULL) {
        return -1;
    }

    if (PyModule_AddObjectRef(module, "DefaultEnum", enum_def) < 0) {
        Py_DECREF(enum_def);
        return -1;
    }
    return 0;
}
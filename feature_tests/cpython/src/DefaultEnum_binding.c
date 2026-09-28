#include "include/DefaultEnum.h"

#include "diplomat_runtime_common.h"

int DefaultEnum_init(PyObject* module) {
    PyObject* enum_name = PyUnicode_FromString("DefaultEnum");
    // PyTuple_Pack creates strong references to the provided objects (this is equivalent to Py_BuildValue("(O)", enum_name)):
    PyObject* enum_args = PyTuple_Pack(1, enum_name);
    // So we must decref:
    Py_DECREF(enum_name);

    if (enum_args == NULL) {
        return -1;
    }

    PyObject* enum_kwargs = PyDict_New();

    PyObject* enum_names = PyTuple_New(2);
    
    PyObject* A_name = PyUnicode_FromString("A");
    // SetItem setals, so we do not need to decref:
    PyTuple_SetItem(enum_names, 0, A_name);

    
    PyObject* B_name = PyUnicode_FromString("B");
    // SetItem setals, so we do not need to decref:
    PyTuple_SetItem(enum_names, 1, B_name);

    

    PyDict_SetItemString(enum_kwargs, "names", enum_names);

    // Python enums can be passed without us needing to hold a reference, so we just create straight from the `Enum` class:
    PyObject* enum_def = PyObject_Call(diplomat_internals.enum_ty, enum_args, enum_kwargs);
    Py_DECREF(enum_args);
    Py_DECREF(enum_names);
    Py_DECREF(enum_kwargs);
    if (enum_def == NULL) {
        return -1;
    }

    if (PyModule_AddObjectRef(module, "DefaultEnum", enum_def) < 0) {
        Py_DECREF(enum_def);
        return -1;
    }
    return 0;
}
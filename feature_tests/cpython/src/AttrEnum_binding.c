#include "include/AttrEnum.h"

#include "diplomat_runtime_common.h"

int AttrEnum_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(3);
    
    PyObject* A_value = PyLong_FromLong(0);
    if (A_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* A_name = PyUnicode_FromString("A");
    
    PyObject* A_name_and_value = PyTuple_Pack(2, A_name, A_value);
    Py_DECREF(A_name);
    Py_DECREF(A_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 0, A_name_and_value);
    
    PyObject* B_value = PyLong_FromLong(1);
    if (B_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* B_name = PyUnicode_FromString("B");
    
    PyObject* B_name_and_value = PyTuple_Pack(2, B_name, B_value);
    Py_DECREF(B_name);
    Py_DECREF(B_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 1, B_name_and_value);
    
    PyObject* C_value = PyLong_FromLong(2);
    if (C_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* C_name = PyUnicode_FromString("Renamed");
    
    PyObject* C_name_and_value = PyTuple_Pack(2, C_name, C_value);
    Py_DECREF(C_name);
    Py_DECREF(C_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 2, C_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("AttrEnum");
    // PyTuple_Pack creates strong references to the provided objects (this is equivalent to Py_BuildValue("(O)", enum_name)):
    PyObject* enum_args = PyTuple_Pack(2, enum_name, enum_names_values);
    // So we must decref the args:
    Py_DECREF(enum_name);
    Py_DECREF(enum_names_values);

    if (enum_args == NULL) {
        return -1;
    }

    // Python enums can be passed without us needing to hold a reference, so we just create straight from the `Enum` class:
    PyObject* enum_def = PyObject_Call(diplomat_internals.enum_ty, enum_args, NULL);
    Py_DECREF(enum_args);
    if (enum_def == NULL) {
        return -1;
    }

    int enum_add_result = PyModule_AddObjectRef(module, "AttrEnum", enum_def);
    Py_DECREF(enum_def);
    return enum_add_result;
}
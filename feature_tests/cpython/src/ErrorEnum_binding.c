#include "include/ErrorEnum.h"

#include "diplomat_runtime_common.h"

int ErrorEnum_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(2);
    
    PyObject* Foo_value = PyLong_FromLong(0);
    if (Foo_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Foo_name = PyUnicode_FromString("Foo");
    
    PyObject* Foo_name_and_value = PyTuple_Pack(2, Foo_name, Foo_value);
    Py_DECREF(Foo_name);
    Py_DECREF(Foo_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 0, Foo_name_and_value);
    
    PyObject* Bar_value = PyLong_FromLong(1);
    if (Bar_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Bar_name = PyUnicode_FromString("Bar");
    
    PyObject* Bar_name_and_value = PyTuple_Pack(2, Bar_name, Bar_value);
    Py_DECREF(Bar_name);
    Py_DECREF(Bar_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 1, Bar_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("ErrorEnum");
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

    int enum_add_result = PyModule_AddObjectRef(module, "ErrorEnum", enum_def);
    Py_DECREF(enum_def);
    return enum_add_result;
}
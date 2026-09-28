
#include "include/ErrorEnum_Py.h"

static PyObject* enumClass = NULL;

int ErrorEnum_to_variant(PyObject* enum_obj, ErrorEnum* enum_out) {
    if (enumClass == NULL) {
        // TODO: Repr the enum object.
        PyErr_SetString(PyExc_RuntimeError, "Converting to ErrorEnum failed because it was not initialized (Did you try importing ErrorEnum?)");
        return -1;
    }
    if (PyObject_IsInstance(enum_obj, enumClass) == -1) {
        return -1;
    }

    PyObject* enum_val = PyObject_GetAttrString(enum_obj, "value");
    if (enum_val == NULL) {
        return -1;
    }

    // FIXME: This should change based on the bit width of the enum (and whether it's signed or not)
    int result = PyLong_AsInt64(enum_val, enum_out);
    Py_DECREF(enum_val);
    if (result == -1) {
        return -1;
    }

    return 0;
}

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
    enumClass = PyObject_Call(diplomat_internals.enum_ty, enum_args, NULL);
    Py_DECREF(enum_args);
    if (enumClass == NULL) {
        return -1;
    }

    int enum_add_result = PyModule_AddObjectRef(module, "ErrorEnum", enumClass);
    Py_DECREF(enumClass);
    return enum_add_result;
}

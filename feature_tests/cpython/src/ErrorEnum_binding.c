
#include "include/ErrorEnum_Py.h"

PyObject* enum_class;

PyObject* ErrorEnum_to_py(ErrorEnum enum_in) {
    if (enum_class == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "enum_class is NULL, most likely ErrorEnum did not initialize");
        return NULL;
    }

    PyObject* enum_arg = PyLong_FromLong((long)enum_in);
    if (enum_arg == NULL) {
        return NULL;
    }

    PyObject* py_out = PyObject_CallOneArg(enum_class, enum_arg);
    Py_DECREF(enum_arg);
    return py_out;
}

int ErrorEnum_from_py(PyObject* enum_obj, ErrorEnum* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of ErrorEnum");
        return -1;
    }

    PyObject* value = PyObject_GetAttrString(enum_obj, "value");
    if (!value) {
        return -1;
    }
    
    long enum_val = PyLong_AsLong(value);
    Py_DECREF(value);
    if (PyErr_Occurred()) {
        return -1;
    }

    // FIXME: Maybe a manual conversion? Need to validate the bounds of the enum.
    *enum_out = (ErrorEnum)enum_val;
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
    enum_class = PyObject_Call(diplomat_internals.enum_ty, enum_args, NULL);
    Py_DECREF(enum_args);
    if (enum_class == NULL) {
        return -1;
    }

    

    int enum_add_result = PyModule_AddObjectRef(module, "ErrorEnum", enum_class);
    Py_DECREF(enum_class);
    return enum_add_result;
}

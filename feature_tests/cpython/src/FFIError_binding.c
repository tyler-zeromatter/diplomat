
#include "include/FFIError_Py.h"

static PyObject* enum_class;

PyObject* FFIError_to_py(FFIError enum_in) {
    if (enum_class == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "enum_class is NULL, most likely FFIError did not initialize");
        return NULL;
    }

    // TODO: Verify bounds instead of casting
    PyObject* enum_arg = PyLong_FromLong((long)enum_in);
    if (enum_arg == NULL) {
        return NULL;
    }

    PyObject* py_out = PyObject_CallOneArg(enum_class, enum_arg);
    Py_DECREF(enum_arg);
    return py_out;
}

int FFIError_from_py(PyObject* enum_obj, FFIError* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of FFIError");
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
    *enum_out = (FFIError)enum_val;
    return 0;
}



int FFIError_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(2);
    
    PyObject* FFI_value = PyLong_FromLong(0);
    if (FFI_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* FFI_name = PyUnicode_FromString("FFI");
    
    PyObject* FFI_name_and_value = PyTuple_Pack(2, FFI_name, FFI_value);
    Py_DECREF(FFI_name);
    Py_DECREF(FFI_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 0, FFI_name_and_value);
    
    PyObject* User_value = PyLong_FromLong(1);
    if (User_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* User_name = PyUnicode_FromString("User");
    
    PyObject* User_name_and_value = PyTuple_Pack(2, User_name, User_value);
    Py_DECREF(User_name);
    Py_DECREF(User_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 1, User_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("FfiError");
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

    

    int enum_add_result = PyModule_AddObjectRef(module, "FfiError", enum_class);
    Py_DECREF(enum_class);
    return enum_add_result;
}


#include "include/UnimportedEnum_Py.h"

static PyObject* enum_class;

PyObject* UnimportedEnum_to_py(UnimportedEnum enum_in) {
    if (enum_class == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "enum_class is NULL, most likely UnimportedEnum did not initialize");
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

int UnimportedEnum_from_py(PyObject* enum_obj, UnimportedEnum* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of UnimportedEnum");
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
    *enum_out = (UnimportedEnum)enum_val;
    return 0;
}



int UnimportedEnum_init(PyObject* module) {
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
    PyObject* C_name = PyUnicode_FromString("C");
    
    PyObject* C_name_and_value = PyTuple_Pack(2, C_name, C_value);
    Py_DECREF(C_name);
    Py_DECREF(C_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 2, C_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("UnimportedEnum");
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

    

    int enum_add_result = PyModule_AddObjectRef(module, "UnimportedEnum", enum_class);
    Py_DECREF(enum_class);
    return enum_add_result;
}

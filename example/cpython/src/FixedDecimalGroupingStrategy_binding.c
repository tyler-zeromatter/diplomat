
#include "include/FixedDecimalGroupingStrategy_Py.h"

static PyObject* enum_class;

PyObject* FixedDecimalGroupingStrategy_to_py(FixedDecimalGroupingStrategy enum_in) {
    if (enum_class == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "enum_class is NULL, most likely FixedDecimalGroupingStrategy did not initialize");
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

int FixedDecimalGroupingStrategy_from_py(PyObject* enum_obj, FixedDecimalGroupingStrategy* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of FixedDecimalGroupingStrategy");
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
    *enum_out = (FixedDecimalGroupingStrategy)enum_val;
    return 0;
}



int FixedDecimalGroupingStrategy_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(4);
    
    PyObject* Auto_value = PyLong_FromLong(0);
    if (Auto_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Auto_name = PyUnicode_FromString("Auto");
    
    PyObject* Auto_name_and_value = PyTuple_Pack(2, Auto_name, Auto_value);
    Py_DECREF(Auto_name);
    Py_DECREF(Auto_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 0, Auto_name_and_value);
    
    PyObject* Never_value = PyLong_FromLong(1);
    if (Never_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Never_name = PyUnicode_FromString("Never");
    
    PyObject* Never_name_and_value = PyTuple_Pack(2, Never_name, Never_value);
    Py_DECREF(Never_name);
    Py_DECREF(Never_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 1, Never_name_and_value);
    
    PyObject* Always_value = PyLong_FromLong(2);
    if (Always_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Always_name = PyUnicode_FromString("Always");
    
    PyObject* Always_name_and_value = PyTuple_Pack(2, Always_name, Always_value);
    Py_DECREF(Always_name);
    Py_DECREF(Always_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 2, Always_name_and_value);
    
    PyObject* Min2_value = PyLong_FromLong(3);
    if (Min2_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* Min2_name = PyUnicode_FromString("Min2");
    
    PyObject* Min2_name_and_value = PyTuple_Pack(2, Min2_name, Min2_value);
    Py_DECREF(Min2_name);
    Py_DECREF(Min2_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 3, Min2_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("icu4x.FixedDecimalGroupingStrategy");
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

    

    int enum_add_result = PyModule_AddObjectRef(module, "FixedDecimalGroupingStrategy", enum_class);
    Py_DECREF(enum_class);
    return enum_add_result;
}

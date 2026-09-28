#include "include/MyEnum.h"

#include "diplomat_runtime_common.h"

int MyEnum_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(6);
    
    PyObject* A_value = PyLong_FromLong(-2);
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
    
    PyObject* B_value = PyLong_FromLong(-1);
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
    
    PyObject* C_value = PyLong_FromLong(0);
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
    
    PyObject* D_value = PyLong_FromLong(1);
    if (D_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* D_name = PyUnicode_FromString("D");
    
    PyObject* D_name_and_value = PyTuple_Pack(2, D_name, D_value);
    Py_DECREF(D_name);
    Py_DECREF(D_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 3, D_name_and_value);
    
    PyObject* E_value = PyLong_FromLong(2);
    if (E_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* E_name = PyUnicode_FromString("E");
    
    PyObject* E_name_and_value = PyTuple_Pack(2, E_name, E_value);
    Py_DECREF(E_name);
    Py_DECREF(E_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 4, E_name_and_value);
    
    PyObject* F_value = PyLong_FromLong(3);
    if (F_value == NULL) {
        Py_DECREF(enum_names_values);
        return -1;
    }
    PyObject* F_name = PyUnicode_FromString("F");
    
    PyObject* F_name_and_value = PyTuple_Pack(2, F_name, F_value);
    Py_DECREF(F_name);
    Py_DECREF(F_value);

    // SetItem steals, so we do not need to decref:
    PyTuple_SetItem(enum_names_values, 5, F_name_and_value);
    

    PyObject* enum_name = PyUnicode_FromString("MyEnum");
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

    int enum_add_result = PyModule_AddObjectRef(module, "MyEnum", enum_def);
    Py_DECREF(enum_def);
    return enum_add_result;
}
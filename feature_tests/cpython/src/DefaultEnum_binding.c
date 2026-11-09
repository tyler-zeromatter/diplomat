
#include "include/DefaultEnum_Py.h"

static PyObject* enum_class;

PyObject* DefaultEnum_to_py(DefaultEnum enum_in) {
    if (enum_class == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "enum_class is NULL, most likely DefaultEnum did not initialize");
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

int DefaultEnum_from_py(PyObject* enum_obj, DefaultEnum* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of DefaultEnum");
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
    *enum_out = (DefaultEnum)enum_val;
    return 0;
}


PyObject* __new__py_meth(PyObject* self,  PyObject* const *args, Py_ssize_t nargs, PyObject* kwnames){
    if (nargs != 0) {
        PyErr_Format(PyExc_TypeError, "new takes 0 arguments, got %lu", nargs);
        return NULL;
    }

    

    

    DefaultEnum out = DefaultEnum_new();
    PyObject* py_out = NULL;
    
    
    py_out = DefaultEnum_to_py(out);
    
    return py_out;
}

static PyMethodDef __new__py_def = {
    // FIXME: Method name should be lib_name.method_name.name
    .ml_name = "new",
    .ml_meth = __new__py_meth,
    // TODO: Kwargs construction, non-method flag for functions
    .ml_flags =  METH_FASTCALL | METH_KEYWORDS,
    // TODO:
    .ml_doc = "",
};


int DefaultEnum_init(PyObject* module) {
    PyObject* enum_names_values = PyTuple_New(2);
    
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
    

    PyObject* enum_name = PyUnicode_FromString("_somelib.DefaultEnum");
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

    PyObject* __new__py_static_method = PyCMethod_New(&__new__py_def, NULL, module, NULL);
    if (__new__py_static_method == NULL) {
        return -1;
    }

    PyObject* __new__py_method = PyStaticMethod_New(__new__py_static_method);
    if (!__new__py_method) {
        return -1;
    }
    
    Py_DECREF(__new__py_static_method);
    
    PyObject_SetAttrString(enum_class, "new", __new__py_method);
    

    int enum_add_result = PyModule_AddObjectRef(module, "DefaultEnum", enum_class);
    Py_DECREF(enum_class);
    return enum_add_result;
}

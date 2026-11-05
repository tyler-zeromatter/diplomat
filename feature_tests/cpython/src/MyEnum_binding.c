
#include "include/MyEnum_Py.h"

int MyEnum_from_py(PyObject* enum_obj, MyEnum* enum_out) {
    int instance_check = PyObject_IsInstance(enum_obj, diplomat_internals.enum_ty);
    if (instance_check == -1) {
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of MyEnum");
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
    *enum_out = (MyEnum)enum_val;
    return 0;
}


PyObject* __into_value__py_meth(PyObject* self, PyTypeObject* defining_class, PyObject* const *args, Py_ssize_t nargs, PyObject* kwnames){
    if (nargs != 0) {
        PyErr_Format(PyExc_TypeError, "into_value takes 0 arguments, got %lu", nargs);
        return NULL;
    }

    
    if (!self) {
        PyErr_SetString(PyExc_RuntimeError, "No value for self found");
        return NULL;
    }
    MyEnum self_rs;
    if (MyEnum_from_py(self, &self_rs) == -1) {
        return NULL;
    }
    

    

    int8_t out = MyEnum_into_value( self_rs );
    PyObject* py_out = NULL;
    
    
    py_out = Diplomat_long_to_py(out);
    
    return py_out;
}

static PyMethodDef __into_value__py_def = {
    // FIXME: Method name should be lib_name.method_name.name
    .ml_name = "into_value",
    .ml_meth = __into_value__py_meth,
    // TODO: Kwargs construction, non-method flag for functions
    .ml_flags =  METH_METHOD |  METH_FASTCALL | METH_KEYWORDS,
    // TODO:
    .ml_doc = "",
};

PyObject* __get_a__py_meth(PyObject* self,  PyObject* const *args, Py_ssize_t nargs, PyObject* kwnames){
    if (nargs != 0) {
        PyErr_Format(PyExc_TypeError, "get_a takes 0 arguments, got %lu", nargs);
        return NULL;
    }

    

    

    MyEnum out = MyEnum_get_a();
    PyObject* py_out = NULL;
    
    
    return py_out;
}

static PyMethodDef __get_a__py_def = {
    // FIXME: Method name should be lib_name.method_name.name
    .ml_name = "get_a",
    .ml_meth = __get_a__py_meth,
    // TODO: Kwargs construction, non-method flag for functions
    .ml_flags =  METH_FASTCALL | METH_KEYWORDS,
    // TODO:
    .ml_doc = "",
};


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
    PyObject* enum_class = PyObject_Call(diplomat_internals.enum_ty, enum_args, NULL);
    Py_DECREF(enum_args);
    if (enum_class == NULL) {
        return -1;
    }

    PyObject* __into_value__py_method = PyDescr_NewMethod((PyTypeObject*) enum_class, &__into_value__py_def);
    if (!__into_value__py_method) {
        return -1;
    }
    
    PyObject_SetAttrString(enum_class, "into_value", __into_value__py_method);
    PyObject* __get_a__py_static_method = PyCMethod_New(&__get_a__py_def, NULL, module, NULL);
    if (__get_a__py_static_method == NULL) {
        return -1;
    }

    PyObject* __get_a__py_method = PyStaticMethod_New(__get_a__py_static_method);
    if (!__get_a__py_method) {
        return -1;
    }
    
    Py_DECREF(__get_a__py_static_method);
    
    PyObject_SetAttrString(enum_class, "get_a", __get_a__py_method);
    

    int enum_add_result = PyModule_AddObjectRef(module, "MyEnum", enum_class);
    return enum_add_result;
}


#include "include/FixedDecimalFormatterOptions_Py.h"

static PyObject* struct_py = NULL;
static PyObject* (*struct_alloc)(PyTypeObject* self, Py_ssize_t nitems) = NULL;

PyObject* FixedDecimalFormatterOptions_to_py(FixedDecimalFormatterOptions in) {
    if (struct_py == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "Could not get FixedDecimalFormatterOptions Python type, module may not be initialized.");
        return NULL;
    }

    if (struct_alloc == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "Could not get FixedDecimalFormatterOptions allocator, module may not be initialized.");
        return NULL;
    }

    // We use the built-in allocator because the constructor may have been overridden:
    PyObject* struct_obj = struct_alloc(struct_py, 1);
    if (struct_obj == NULL) {
        return NULL;
    }
    

    FixedDecimalFormatterOptions* self_data = (FixedDecimalFormatterOptions*)PyObject_GetTypeData(struct_obj, struct_py);
    if (self_data == NULL) {
        Py_DECREF(struct_obj);
        return NULL;
    }

    *self_data = in;

    struct_obj = PyObject_Init(struct_obj, struct_py);
    if (struct_obj == NULL) {
        return NULL;
    }
    return struct_obj;
}

int FixedDecimalFormatterOptions_from_py(PyObject* struct_obj, FixedDecimalFormatterOptions** struct_out) {
    int instance_check = PyObject_IsInstance(struct_obj, struct_py);
    if (instance_check == -1) {
        Py_DECREF(struct_obj);
        return -1;
    } else if (instance_check == 0) {
        PyErr_SetString(PyExc_TypeError, "Self type is not an instance of FixedDecimalFormatterOptions");
        Py_DECREF(struct_obj);
        return -1;
    }

    FixedDecimalFormatterOptions* self_data = (FixedDecimalFormatterOptions*)PyObject_GetTypeData(struct_obj, struct_py);
    Py_DECREF(struct_obj);
    *struct_out = self_data;

    return 0;
}




PyObject* __default__py_meth(PyObject* self,  PyObject* const *args, Py_ssize_t nargs, PyObject* kwnames){
    if (nargs != 0) {
        PyErr_Format(PyExc_TypeError, "default takes 0 arguments, got %lu", nargs);
        return NULL;
    }

    

    

    FixedDecimalFormatterOptions out = icu4x_FixedDecimalFormatterOptions_default_mv1();
    PyObject* py_out = NULL;
    
    
    py_out = FixedDecimalFormatterOptions_to_py(out);
    
    return py_out;
}



static PyMethodDef struct_tp_methods[] = {
    {
        .ml_name = "default",
        .ml_meth = __default__py_meth,
        // TODO: Kwargs construction, non-method flag for functions
        .ml_flags =  METH_STATIC  | METH_FASTCALL | METH_KEYWORDS,
        // TODO:
        .ml_doc = "",
    },
    {NULL}
};

static PyType_Slot struct_slots[] = {
    {Py_tp_methods, struct_tp_methods},
    {0, NULL},
};

static PyType_Spec struct_spec = {
    // TODO: Add module name
    .name = "icu4x.FixedDecimalFormatterOptions",
    // Negative because this is in *addition* to the base PyObject type:
    .basicsize = -sizeof(FixedDecimalFormatterOptions),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = struct_slots,
};

int FixedDecimalFormatterOptions_init(PyObject* module) {
    struct_py = PyType_FromModuleAndSpec(module, &struct_spec, NULL);

    if (struct_py == NULL) {
        return -1;
    }

    struct_alloc = PyType_GetSlot((PyTypeObject*)struct_py, Py_tp_alloc);
    if (struct_alloc == NULL) {
        PyErr_SetString(PyExc_ImportError, "Could not get allocator for FixedDecimalFormatterOptions");
        return -1;
    }

    PyModule_AddObjectRef(module, "FixedDecimalFormatterOptions", struct_py);
    Py_DECREF(struct_py);

    return 0;
}

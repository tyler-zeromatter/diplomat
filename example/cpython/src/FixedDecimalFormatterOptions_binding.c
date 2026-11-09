
#include "include/FixedDecimalFormatterOptions_Py.h"




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
        .ml_name = "somelib.FixedDecimalFormatterOptions.default",
        .ml_meth = __default__py_meth,
        // TODO: Kwargs construction, non-method flag for functions
        .ml_flags =  METH_FASTCALL | METH_KEYWORDS,
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
    .name = "somelib.FixedDecimalFormatterOptions",
    // Negative because this is in *addition* to the base PyObject type:
    .basicsize = -sizeof(FixedDecimalFormatterOptions),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = struct_slots,
};

int FixedDecimalFormatterOptions_init(PyObject* module) {
    PyObject* struct_py = PyType_FromModuleAndSpec(module, &struct_spec, NULL);

    if (struct_py == NULL) {
        return -1;
    }

    PyModule_AddObjectRef(module, "FixedDecimalFormatterOptions", struct_py);

    return 0;
}

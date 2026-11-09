#include "diplomat_runtime_common.h"

#include "include/FixedDecimalFormatterOptions_Py.h"

#include "include/FixedDecimalGroupingStrategy_Py.h"


static void somelib_free(void* self) {
    if (diplomat_internals.enum_mod) {
        Py_DECREF(diplomat_internals.enum_mod);
    }

    if (diplomat_internals.enum_ty) {
        Py_DECREF(diplomat_internals.enum_ty);
    }
}

#define CHECK(check, reason, string) if (check) { PyErr_SetString(reason, string); somelib_free((void*)mod); return -1; }

static int somelib_exec(PyObject* mod) {
    if (diplomat_internals.initialized) {
        PyErr_SetString(PyExc_ImportError, "cannot initialize `somelib` more than once");
        return -1;
    }

    // Grab enums:
    diplomat_internals.enum_mod = PyImport_ImportModule("enum");
    CHECK(!diplomat_internals.enum_mod, PyExc_ImportError, "could not find `enum` module");

    diplomat_internals.enum_ty = PyObject_GetAttrString(diplomat_internals.enum_mod, "Enum");
    CHECK(!diplomat_internals.enum_ty, PyExc_ImportError, "could not find `enum.Enum` type");

    
    if (FixedDecimalFormatterOptions_init(mod) < 0) {
        somelib_free((void*)mod);
        return -1;
    }
    
    if (FixedDecimalGroupingStrategy_init(mod) < 0) {
        somelib_free((void*)mod);
        return -1;
    }
    

    diplomat_internals.initialized = true;
    return 0;
}

Internals diplomat_internals = {};

static PyModuleDef_Slot somelib_module_slots[] = {
    {Py_mod_exec, somelib_exec},
    {0, NULL}
};

// See https://docs.python.org/3/c-api/module.html#c.PyModuleDef
static struct PyModuleDef somelib_module = {
    // m_base
    PyModuleDef_HEAD_INIT,
    // m_name
    "somelib",
    // m_doc
    "",
    // m_size
    0,
    // m_methods
    NULL,
    // m_slots
    somelib_module_slots,
    // m_traverse
    NULL,
    // m_clear
    NULL,
    // m_free
    somelib_free,
};

PyMODINIT_FUNC PyInit_somelib() {
    return PyModuleDef_Init(&somelib_module);
}
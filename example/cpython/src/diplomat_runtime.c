#include "diplomat_runtime_common.h"

#include "include/FixedDecimalFormatterOptions_Py.h"

#include "include/FixedDecimalGroupingStrategy_Py.h"


static void icu4x_free(void* self) {
    if (diplomat_internals.enum_mod) {
        Py_DECREF(diplomat_internals.enum_mod);
    }

    if (diplomat_internals.enum_ty) {
        Py_DECREF(diplomat_internals.enum_ty);
    }
}

#define CHECK(check, reason, string) if (check) { PyErr_SetString(reason, string); icu4x_free((void*)mod); return -1; }

static int icu4x_exec(PyObject* mod) {
    if (diplomat_internals.initialized) {
        PyErr_SetString(PyExc_ImportError, "cannot initialize `icu4x` more than once");
        return -1;
    }

    // Grab enums:
    diplomat_internals.enum_mod = PyImport_ImportModule("enum");
    CHECK(!diplomat_internals.enum_mod, PyExc_ImportError, "could not find `enum` module");

    diplomat_internals.enum_ty = PyObject_GetAttrString(diplomat_internals.enum_mod, "Enum");
    CHECK(!diplomat_internals.enum_ty, PyExc_ImportError, "could not find `enum.Enum` type");

    
    if (FixedDecimalFormatterOptions_init(mod) < 0) {
        icu4x_free((void*)mod);
        return -1;
    }
    
    if (FixedDecimalGroupingStrategy_init(mod) < 0) {
        icu4x_free((void*)mod);
        return -1;
    }
    

    diplomat_internals.initialized = true;
    return 0;
}

Internals diplomat_internals = {};

static PyModuleDef_Slot icu4x_module_slots[] = {
    {Py_mod_exec, icu4x_exec},
    {0, NULL}
};

// See https://docs.python.org/3/c-api/module.html#c.PyModuleDef
static struct PyModuleDef icu4x_module = {
    // m_base
    PyModuleDef_HEAD_INIT,
    // m_name
    "icu4x",
    // m_doc
    "",
    // m_size
    0,
    // m_methods
    NULL,
    // m_slots
    icu4x_module_slots,
    // m_traverse
    NULL,
    // m_clear
    NULL,
    // m_free
    icu4x_free,
};

PyMODINIT_FUNC PyInit_icu4x() {
    return PyModuleDef_Init(&icu4x_module);
}
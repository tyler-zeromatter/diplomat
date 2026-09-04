#include "diplomat_runtime_common.h"

static int _somelib_exec(PyObject* m) {
    return 0;
} 

static PyModuleDef_Slot _somelib_module_slots[] = {
    {Py_mod_exec, _somelib_exec},
    {0, NULL}
};

// See https://docs.python.org/3/c-api/module.html#c.PyModuleDef
static struct PyModuleDef _somelib_module = {
    // m_base
    PyModuleDef_HEAD_INIT,
    // m_name
    "_somelib",
    // m_doc
    "",
    // m_size
    0,
    // m_methods
    NULL,
    // m_slots
    _somelib_module_slots,
    // m_traverse
    NULL,
    // m_clear
    NULL,
    // m_free,
    NULL
};

PyMODINIT_FUNC PyInit__somelib() {
    return PyModuleDef_Init(&_somelib_module);
}
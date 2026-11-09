
#include "include/TestMacroStruct_Py.h"

static PyType_Slot struct_slots[] = {
    {0, NULL},
};

static PyType_Spec struct_spec = {
    // TODO: Add module name
    .name = "_somelib.RenamedTestMacroStruct",
    // Negative because this is in *addition* to the base PyObject type:
    .basicsize = -sizeof(TestMacroStruct),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = struct_slots,
};

int TestMacroStruct_init(PyObject* module) {
    PyObject* struct_py = PyType_FromModuleAndSpec(module, &struct_spec, NULL);

    if (struct_py == NULL) {
        return -1;
    }

    PyModule_AddObjectRef(module, "TestMacroStruct", struct_py);

    return 0;
}

#include "diplomat_runtime_common.h"

static int _somelib_exec(PyObject* mod) {
    
    if (DeprecatedStruct_init(mod) < 0) {
        return -1;
    }
    
    if (RenamedCachedIncludeZST_init(mod) < 0) {
        return -1;
    }
    
    if (StructWithAttrs_init(mod) < 0) {
        return -1;
    }
    
    if (TestMacroStruct_init(mod) < 0) {
        return -1;
    }
    
    if (ImportedStruct_init(mod) < 0) {
        return -1;
    }
    
    if (BorrowedFields_init(mod) < 0) {
        return -1;
    }
    
    if (BorrowedFieldsReturning_init(mod) < 0) {
        return -1;
    }
    
    if (BorrowedFieldsWithBounds_init(mod) < 0) {
        return -1;
    }
    
    if (NestedBorrowedFields_init(mod) < 0) {
        return -1;
    }
    
    if (CachedIncludeZST_init(mod) < 0) {
        return -1;
    }
    
    if (ErrorStruct_init(mod) < 0) {
        return -1;
    }
    
    if (CyclicStructA_init(mod) < 0) {
        return -1;
    }
    
    if (CyclicStructB_init(mod) < 0) {
        return -1;
    }
    
    if (CyclicStructC_init(mod) < 0) {
        return -1;
    }
    
    if (MyStruct_init(mod) < 0) {
        return -1;
    }
    
    if (MyZst_init(mod) < 0) {
        return -1;
    }
    
    if (PrimitiveStruct_init(mod) < 0) {
        return -1;
    }
    
    if (StructOfOpaque_init(mod) < 0) {
        return -1;
    }
    
    if (StructWithSlices_init(mod) < 0) {
        return -1;
    }
    
    if (AttrEnum_init(mod) < 0) {
        return -1;
    }
    
    if (DeprecatedEnum_init(mod) < 0) {
        return -1;
    }
    
    if (UnimportedEnum_init(mod) < 0) {
        return -1;
    }
    
    if (ErrorEnum_init(mod) < 0) {
        return -1;
    }
    
    if (ContiguousEnum_init(mod) < 0) {
        return -1;
    }
    
    if (DefaultEnum_init(mod) < 0) {
        return -1;
    }
    
    if (MyEnum_init(mod) < 0) {
        return -1;
    }
    
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
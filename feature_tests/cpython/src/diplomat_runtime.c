#include "diplomat_runtime_common.h"

static void _somelib_free(void* self) {
    if (diplomat_internals.enum_mod) {
        Py_DECREF(diplomat_internals.enum_mod);
    }

    if (diplomat_internals.enum_ty) {
        Py_DECREF(diplomat_internals.enum_ty);
    }
}

#define CHECK(check, reason, string) if (check) { PyErr_SetString(reason, string); _somelib_free((void*)mod); return -1; }

static int _somelib_exec(PyObject* mod) {
    if (diplomat_internals.initialized) {
        PyErr_SetString(PyExc_ImportError, "cannot initialize `_somelib` more than once");
        return -1;
    }

    // Grab enums:
    diplomat_internals.enum_mod = PyImport_ImportModule("enum");
    CHECK(!diplomat_internals.enum_mod, PyExc_ImportError, "could not find `enum` module");

    diplomat_internals.enum_ty = PyObject_GetAttrString(diplomat_internals.enum_mod, "Enum");
    CHECK(!diplomat_internals.enum_ty, PyExc_ImportError, "could not find `enum.Enum` type");

    
    CHECK(DeprecatedStruct_init(mod) < 0, PyExc_ImportError, "could not initialize DeprecatedStruct_init");
    
    CHECK(RenamedCachedIncludeZST_init(mod) < 0, PyExc_ImportError, "could not initialize RenamedCachedIncludeZST_init");
    
    CHECK(StructWithAttrs_init(mod) < 0, PyExc_ImportError, "could not initialize StructWithAttrs_init");
    
    CHECK(TestMacroStruct_init(mod) < 0, PyExc_ImportError, "could not initialize TestMacroStruct_init");
    
    CHECK(ImportedStruct_init(mod) < 0, PyExc_ImportError, "could not initialize ImportedStruct_init");
    
    CHECK(BorrowedFields_init(mod) < 0, PyExc_ImportError, "could not initialize BorrowedFields_init");
    
    CHECK(BorrowedFieldsReturning_init(mod) < 0, PyExc_ImportError, "could not initialize BorrowedFieldsReturning_init");
    
    CHECK(BorrowedFieldsWithBounds_init(mod) < 0, PyExc_ImportError, "could not initialize BorrowedFieldsWithBounds_init");
    
    CHECK(NestedBorrowedFields_init(mod) < 0, PyExc_ImportError, "could not initialize NestedBorrowedFields_init");
    
    CHECK(CachedIncludeZST_init(mod) < 0, PyExc_ImportError, "could not initialize CachedIncludeZST_init");
    
    CHECK(ErrorStruct_init(mod) < 0, PyExc_ImportError, "could not initialize ErrorStruct_init");
    
    CHECK(CyclicStructA_init(mod) < 0, PyExc_ImportError, "could not initialize CyclicStructA_init");
    
    CHECK(CyclicStructB_init(mod) < 0, PyExc_ImportError, "could not initialize CyclicStructB_init");
    
    CHECK(CyclicStructC_init(mod) < 0, PyExc_ImportError, "could not initialize CyclicStructC_init");
    
    CHECK(MyStruct_init(mod) < 0, PyExc_ImportError, "could not initialize MyStruct_init");
    
    CHECK(MyZst_init(mod) < 0, PyExc_ImportError, "could not initialize MyZst_init");
    
    CHECK(PrimitiveStruct_init(mod) < 0, PyExc_ImportError, "could not initialize PrimitiveStruct_init");
    
    CHECK(StructOfOpaque_init(mod) < 0, PyExc_ImportError, "could not initialize StructOfOpaque_init");
    
    CHECK(StructWithSlices_init(mod) < 0, PyExc_ImportError, "could not initialize StructWithSlices_init");
    
    CHECK(AttrEnum_init(mod) < 0, PyExc_ImportError, "could not initialize AttrEnum_init");
    
    CHECK(DeprecatedEnum_init(mod) < 0, PyExc_ImportError, "could not initialize DeprecatedEnum_init");
    
    CHECK(UnimportedEnum_init(mod) < 0, PyExc_ImportError, "could not initialize UnimportedEnum_init");
    
    CHECK(ErrorEnum_init(mod) < 0, PyExc_ImportError, "could not initialize ErrorEnum_init");
    
    CHECK(ContiguousEnum_init(mod) < 0, PyExc_ImportError, "could not initialize ContiguousEnum_init");
    
    CHECK(DefaultEnum_init(mod) < 0, PyExc_ImportError, "could not initialize DefaultEnum_init");
    
    CHECK(MyEnum_init(mod) < 0, PyExc_ImportError, "could not initialize MyEnum_init");
    

    diplomat_internals.initialized = true;
    return 0;
}

Internals diplomat_internals = {};

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
    // m_free
    _somelib_free,
};

PyMODINIT_FUNC PyInit__somelib() {
    return PyModuleDef_Init(&_somelib_module);
}
#include "diplomat_runtime_common.h"

#include "include/DeprecatedStruct_Py.h"

#include "include/RenamedCachedIncludeZST_Py.h"

#include "include/StructWithAttrs_Py.h"

#include "include/TestMacroStruct_Py.h"

#include "include/ImportedStruct_Py.h"

#include "include/BorrowedFields_Py.h"

#include "include/BorrowedFieldsReturning_Py.h"

#include "include/BorrowedFieldsWithBounds_Py.h"

#include "include/NestedBorrowedFields_Py.h"

#include "include/CachedIncludeZST_Py.h"

#include "include/ErrorStruct_Py.h"

#include "include/CyclicStructA_Py.h"

#include "include/CyclicStructB_Py.h"

#include "include/CyclicStructC_Py.h"

#include "include/MyStruct_Py.h"

#include "include/MyZst_Py.h"

#include "include/PrimitiveStruct_Py.h"

#include "include/StructOfOpaque_Py.h"

#include "include/StructWithSlices_Py.h"

#include "include/AttrEnum_Py.h"

#include "include/DeprecatedEnum_Py.h"

#include "include/FFIError_Py.h"

#include "include/UnimportedEnum_Py.h"

#include "include/ErrorEnum_Py.h"

#include "include/ContiguousEnum_Py.h"

#include "include/DefaultEnum_Py.h"

#include "include/MyEnum_Py.h"


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

    
    if (DeprecatedStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (RenamedCachedIncludeZST_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (StructWithAttrs_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (TestMacroStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (ImportedStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (BorrowedFields_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (BorrowedFieldsReturning_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (BorrowedFieldsWithBounds_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (NestedBorrowedFields_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (CachedIncludeZST_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (ErrorStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (CyclicStructA_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (CyclicStructB_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (CyclicStructC_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (MyStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (MyZst_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (PrimitiveStruct_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (StructOfOpaque_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (StructWithSlices_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (AttrEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (DeprecatedEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (FFIError_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (UnimportedEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (ErrorEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (ContiguousEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (DefaultEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    
    if (MyEnum_init(mod) < 0) {
        _somelib_free((void*)mod);
        return -1;
    }
    

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
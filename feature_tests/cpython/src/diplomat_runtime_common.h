#ifndef DIPLOMAT_RUNTIME_COMMON
#define DIPLOMAT_RUNTIME_COMMON

#define PY_SSIZE_T_CLEAN
// TODO: Make this an opt-in.
#define PY_LIMITED_API
#include <Python.h>

#include <stdbool.h>

typedef struct Internals {
    bool initialized;
    
    // Enum construction:
    PyObject* enum_mod;
    PyObject* enum_ty;
} Internals;

extern Internals diplomat_internals;

// Define all the initialization methods:
int DeprecatedStruct_init(PyObject* module);
int RenamedCachedIncludeZST_init(PyObject* module);
int StructWithAttrs_init(PyObject* module);
int TestMacroStruct_init(PyObject* module);
int ImportedStruct_init(PyObject* module);
int BorrowedFields_init(PyObject* module);
int BorrowedFieldsReturning_init(PyObject* module);
int BorrowedFieldsWithBounds_init(PyObject* module);
int NestedBorrowedFields_init(PyObject* module);
int CachedIncludeZST_init(PyObject* module);
int ErrorStruct_init(PyObject* module);
int CyclicStructA_init(PyObject* module);
int CyclicStructB_init(PyObject* module);
int CyclicStructC_init(PyObject* module);
int MyStruct_init(PyObject* module);
int MyZst_init(PyObject* module);
int PrimitiveStruct_init(PyObject* module);
int StructOfOpaque_init(PyObject* module);
int StructWithSlices_init(PyObject* module);
int AttrEnum_init(PyObject* module);
int DeprecatedEnum_init(PyObject* module);
int UnimportedEnum_init(PyObject* module);
int ErrorEnum_init(PyObject* module);
int ContiguousEnum_init(PyObject* module);
int DefaultEnum_init(PyObject* module);
int MyEnum_init(PyObject* module);


#endif
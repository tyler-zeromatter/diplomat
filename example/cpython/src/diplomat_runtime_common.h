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

extern inline PyObject* Diplomat_bool_to_py(bool b);
extern inline PyObject* Diplomat_long_to_py(long l);

#endif
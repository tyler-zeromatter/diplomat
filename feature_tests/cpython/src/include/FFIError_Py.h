
#ifndef FFIError_PY_H
#define FFIError_PY_H

#include "include/FFIError.h"
#include "diplomat_runtime_common.h"

PyObject* FFIError_to_py(FFIError enum_in);
int FFIError_from_py(PyObject* enum_obj, FFIError* enum_out);
int FFIError_init(PyObject* module);

#endif

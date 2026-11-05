
#ifndef FFIError_PY_H
#define FFIError_PY_H

#include "include/FFIError.h"
#include "diplomat_runtime_common.h"

int FFIError_to_variant(PyObject* enum_obj, FFIError* enum_out);
int FFIError_init(PyObject* module);

#endif

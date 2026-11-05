
#ifndef DeprecatedEnum_PY_H
#define DeprecatedEnum_PY_H

#include "include/DeprecatedEnum.h"
#include "diplomat_runtime_common.h"

PyObject* DeprecatedEnum_to_py(DeprecatedEnum enum_in);
int DeprecatedEnum_from_py(PyObject* enum_obj, DeprecatedEnum* enum_out);
int DeprecatedEnum_init(PyObject* module);

#endif


#ifndef DeprecatedEnum_PY_H
#define DeprecatedEnum_PY_H

#include "include/DeprecatedEnum.h"
#include "diplomat_runtime_common.h"

int DeprecatedEnum_to_variant(PyObject* enum_obj, DeprecatedEnum* enum_out);
int DeprecatedEnum_init(PyObject* module);

#endif

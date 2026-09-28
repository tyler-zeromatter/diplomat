
#ifndef ContiguousEnum_PY_H
#define ContiguousEnum_PY_H

#include "include/ContiguousEnum.h"
#include "diplomat_runtime_common.h"

int ContiguousEnum_to_variant(PyObject* enum_obj, ContiguousEnum* enum_out);
int ContiguousEnum_init(PyObject* module);

#endif

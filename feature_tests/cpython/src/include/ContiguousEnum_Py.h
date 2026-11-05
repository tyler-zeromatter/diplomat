
#ifndef ContiguousEnum_PY_H
#define ContiguousEnum_PY_H

#include "include/ContiguousEnum.h"
#include "diplomat_runtime_common.h"

PyObject* ContiguousEnum_to_py(ContiguousEnum enum_in);
int ContiguousEnum_from_py(PyObject* enum_obj, ContiguousEnum* enum_out);
int ContiguousEnum_init(PyObject* module);

#endif

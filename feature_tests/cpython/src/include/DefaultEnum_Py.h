
#ifndef DefaultEnum_PY_H
#define DefaultEnum_PY_H

#include "include/DefaultEnum.h"
#include "diplomat_runtime_common.h"

PyObject* DefaultEnum_to_py(DefaultEnum enum_in);
int DefaultEnum_from_py(PyObject* enum_obj, DefaultEnum* enum_out);
int DefaultEnum_init(PyObject* module);

#endif

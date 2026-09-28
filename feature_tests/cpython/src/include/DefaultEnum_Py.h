
#ifndef DefaultEnum_PY_H
#define DefaultEnum_PY_H

#include "include/DefaultEnum.h"
#include "diplomat_runtime_common.h"

int DefaultEnum_to_variant(PyObject* enum_obj, DefaultEnum* enum_out);
int DefaultEnum_init(PyObject* module);

#endif

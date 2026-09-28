
#ifndef ErrorEnum_PY_H
#define ErrorEnum_PY_H

#include "include/ErrorEnum.h"
#include "diplomat_runtime_common.h"

int ErrorEnum_to_variant(PyObject* enum_obj, ErrorEnum* enum_out);
int ErrorEnum_init(PyObject* module);

#endif

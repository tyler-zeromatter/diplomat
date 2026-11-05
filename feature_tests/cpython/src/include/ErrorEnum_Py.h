
#ifndef ErrorEnum_PY_H
#define ErrorEnum_PY_H

#include "include/ErrorEnum.h"
#include "diplomat_runtime_common.h"

PyObject* ErrorEnum_to_py(ErrorEnum enum_in);
int ErrorEnum_from_py(PyObject* enum_obj, ErrorEnum* enum_out);
int ErrorEnum_init(PyObject* module);

#endif

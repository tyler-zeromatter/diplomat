
#ifndef MyEnum_PY_H
#define MyEnum_PY_H

#include "include/MyEnum.h"
#include "diplomat_runtime_common.h"

int MyEnum_to_variant(PyObject* enum_obj, MyEnum* enum_out);
int MyEnum_init(PyObject* module);

#endif

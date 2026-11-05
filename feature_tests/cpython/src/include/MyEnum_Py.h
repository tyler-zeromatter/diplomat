
#ifndef MyEnum_PY_H
#define MyEnum_PY_H

#include "include/MyEnum.h"
#include "diplomat_runtime_common.h"

PyObject* MyEnum_to_py(MyEnum enum_in);
int MyEnum_from_py(PyObject* enum_obj, MyEnum* enum_out);
int MyEnum_init(PyObject* module);

#endif


#ifndef UnimportedEnum_PY_H
#define UnimportedEnum_PY_H

#include "include/UnimportedEnum.h"
#include "diplomat_runtime_common.h"

PyObject* UnimportedEnum_to_py(UnimportedEnum enum_in);
int UnimportedEnum_from_py(PyObject* enum_obj, UnimportedEnum* enum_out);
int UnimportedEnum_init(PyObject* module);

#endif

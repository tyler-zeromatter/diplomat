
#ifndef UnimportedEnum_PY_H
#define UnimportedEnum_PY_H

#include "include/UnimportedEnum.h"
#include "diplomat_runtime_common.h"

int UnimportedEnum_to_variant(PyObject* enum_obj, UnimportedEnum* enum_out);
int UnimportedEnum_init(PyObject* module);

#endif

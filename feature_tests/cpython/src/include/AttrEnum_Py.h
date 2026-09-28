
#ifndef AttrEnum_PY_H
#define AttrEnum_PY_H

#include "include/AttrEnum.h"
#include "diplomat_runtime_common.h"

int AttrEnum_to_variant(PyObject* enum_obj, AttrEnum* enum_out);
int AttrEnum_init(PyObject* module);

#endif

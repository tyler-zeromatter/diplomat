
#ifndef AttrEnum_PY_H
#define AttrEnum_PY_H

#include "include/AttrEnum.h"
#include "diplomat_runtime_common.h"

PyObject* AttrEnum_to_py(AttrEnum enum_in);
int AttrEnum_from_py(PyObject* enum_obj, AttrEnum* enum_out);
int AttrEnum_init(PyObject* module);

#endif

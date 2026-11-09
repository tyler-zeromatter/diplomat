
#ifndef FixedDecimalGroupingStrategy_PY_H
#define FixedDecimalGroupingStrategy_PY_H

#include "include/FixedDecimalGroupingStrategy.h"
#include "diplomat_runtime_common.h"

PyObject* FixedDecimalGroupingStrategy_to_py(FixedDecimalGroupingStrategy enum_in);
int FixedDecimalGroupingStrategy_from_py(PyObject* enum_obj, FixedDecimalGroupingStrategy* enum_out);
int FixedDecimalGroupingStrategy_init(PyObject* module);

#endif

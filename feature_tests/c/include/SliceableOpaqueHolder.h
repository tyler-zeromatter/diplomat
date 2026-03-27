#ifndef SliceableOpaqueHolder_H
#define SliceableOpaqueHolder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SliceableOpaque.d.h"

#include "SliceableOpaqueHolder.d.h"






SliceableOpaqueHolder* SliceableOpaqueHolder_new(const SliceableOpaque* sl);

DiplomatSliceableOpaqueView SliceableOpaqueHolder_mismatch_lt_ret(const SliceableOpaqueHolder* self);

void SliceableOpaqueHolder_destroy(SliceableOpaqueHolder* self);





#endif // SliceableOpaqueHolder_H

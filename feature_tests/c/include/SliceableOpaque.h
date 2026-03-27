#ifndef SliceableOpaque_H
#define SliceableOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SliceableOpaqueHolder.d.h"

#include "SliceableOpaque.d.h"






SliceableOpaque* SliceableOpaque_new(int32_t i);

int32_t SliceableOpaque_num(const SliceableOpaque* self);

void SliceableOpaque_static_in(DiplomatSliceableOpaqueView i, int32_t n);

void SliceableOpaque_non_static_mismatch_in(DiplomatSliceableOpaqueView i, int32_t n);

DiplomatSliceableOpaqueView SliceableOpaque_static_ret(void);

SliceableOpaqueHolder* SliceableOpaque_make_static_holder(void);

void SliceableOpaque_optional_opaque_in(DiplomatSliceableOpaqueView sl, int32_t n);

void SliceableOpaque_destroy(SliceableOpaque* self);





#endif // SliceableOpaque_H

#ifndef SliceableOpaque_D_H
#define SliceableOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct SliceableOpaque SliceableOpaque;


typedef struct DiplomatSliceableOpaqueView {
  const SliceableOpaque** data;
  size_t len;
} DiplomatSliceableOpaqueView;

typedef struct DiplomatSliceableOpaqueViewMut {
  SliceableOpaque** data;
  size_t len;
} DiplomatSliceableOpaqueViewMut;



#endif // SliceableOpaque_D_H

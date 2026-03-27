#ifndef SliceableOpaqueHolder_D_H
#define SliceableOpaqueHolder_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct SliceableOpaqueHolder SliceableOpaqueHolder;


typedef struct DiplomatSliceableOpaqueHolderView {
  const SliceableOpaqueHolder** data;
  size_t len;
} DiplomatSliceableOpaqueHolderView;

typedef struct DiplomatSliceableOpaqueHolderViewMut {
  SliceableOpaqueHolder** data;
  size_t len;
} DiplomatSliceableOpaqueHolderViewMut;



#endif // SliceableOpaqueHolder_D_H

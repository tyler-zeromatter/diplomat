#ifndef PrimitiveStructVec_D_H
#define PrimitiveStructVec_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct PrimitiveStructVec PrimitiveStructVec;


typedef struct DiplomatPrimitiveStructVecView {
  const PrimitiveStructVec** data;
  size_t len;
} DiplomatPrimitiveStructVecView;

typedef struct DiplomatPrimitiveStructVecViewMut {
  PrimitiveStructVec** data;
  size_t len;
} DiplomatPrimitiveStructVecViewMut;



#endif // PrimitiveStructVec_D_H

#ifndef VectorTest_D_H
#define VectorTest_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct VectorTest VectorTest;


typedef struct DiplomatVectorTestView {
  const VectorTest** data;
  size_t len;
} DiplomatVectorTestView;

typedef struct DiplomatVectorTestViewMut {
  VectorTest** data;
  size_t len;
} DiplomatVectorTestViewMut;



#endif // VectorTest_D_H

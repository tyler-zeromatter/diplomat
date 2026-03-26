#ifndef Nested2_D_H
#define Nested2_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Nested2 Nested2;


typedef struct DiplomatNested2View {
  const Nested2** data;
  size_t len;
} DiplomatNested2View;

typedef struct DiplomatNested2ViewMut {
  Nested2** data;
  size_t len;
} DiplomatNested2ViewMut;



#endif // Nested2_D_H

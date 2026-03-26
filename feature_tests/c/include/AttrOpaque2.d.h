#ifndef AttrOpaque2_D_H
#define AttrOpaque2_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct AttrOpaque2 AttrOpaque2;


typedef struct DiplomatAttrOpaque2View {
  const AttrOpaque2** data;
  size_t len;
} DiplomatAttrOpaque2View;

typedef struct DiplomatAttrOpaque2ViewMut {
  AttrOpaque2** data;
  size_t len;
} DiplomatAttrOpaque2ViewMut;



#endif // AttrOpaque2_D_H

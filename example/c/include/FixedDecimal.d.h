#ifndef FixedDecimal_D_H
#define FixedDecimal_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct FixedDecimal FixedDecimal;


typedef struct DiplomatFixedDecimalView {
  const FixedDecimal** data;
  size_t len;
} DiplomatFixedDecimalView;

typedef struct DiplomatFixedDecimalViewMut {
  FixedDecimal** data;
  size_t len;
} DiplomatFixedDecimalViewMut;



#endif // FixedDecimal_D_H

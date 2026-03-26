#ifndef CallbackHolder_D_H
#define CallbackHolder_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CallbackHolder CallbackHolder;


typedef struct DiplomatCallbackHolderView {
  const CallbackHolder** data;
  size_t len;
} DiplomatCallbackHolderView;

typedef struct DiplomatCallbackHolderViewMut {
  CallbackHolder** data;
  size_t len;
} DiplomatCallbackHolderViewMut;



#endif // CallbackHolder_D_H

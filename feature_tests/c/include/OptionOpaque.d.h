#ifndef OptionOpaque_D_H
#define OptionOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct OptionOpaque OptionOpaque;


typedef struct DiplomatOptionOpaqueView {
  const OptionOpaque** data;
  size_t len;
} DiplomatOptionOpaqueView;

typedef struct DiplomatOptionOpaqueViewMut {
  OptionOpaque** data;
  size_t len;
} DiplomatOptionOpaqueViewMut;



#endif // OptionOpaque_D_H

#ifndef DataProvider_D_H
#define DataProvider_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct DataProvider DataProvider;


typedef struct DiplomatDataProviderView {
  const DataProvider** data;
  size_t len;
} DiplomatDataProviderView;

typedef struct DiplomatDataProviderViewMut {
  DataProvider** data;
  size_t len;
} DiplomatDataProviderViewMut;



#endif // DataProvider_D_H

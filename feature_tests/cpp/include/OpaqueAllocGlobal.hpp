#ifndef SOMELIB_OpaqueAllocGlobal_HPP
#define SOMELIB_OpaqueAllocGlobal_HPP

#include "OpaqueAllocGlobal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void OpaqueAllocGlobal_destroy(OpaqueAllocGlobal* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::capi::OpaqueAllocGlobal* somelib::OpaqueAllocGlobal::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueAllocGlobal*>(this);
}

inline somelib::capi::OpaqueAllocGlobal* somelib::OpaqueAllocGlobal::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueAllocGlobal*>(this);
}

inline const somelib::OpaqueAllocGlobal* somelib::OpaqueAllocGlobal::FromFFI(const somelib::capi::OpaqueAllocGlobal* ptr) {
    return reinterpret_cast<const somelib::OpaqueAllocGlobal*>(ptr);
}

inline somelib::OpaqueAllocGlobal* somelib::OpaqueAllocGlobal::FromFFI(somelib::capi::OpaqueAllocGlobal* ptr) {
    return reinterpret_cast<somelib::OpaqueAllocGlobal*>(ptr);
}

inline void somelib::OpaqueAllocGlobal::operator delete(void* ptr) {
    somelib::capi::OpaqueAllocGlobal_destroy(reinterpret_cast<somelib::capi::OpaqueAllocGlobal*>(ptr));
}


#endif // SOMELIB_OpaqueAllocGlobal_HPP

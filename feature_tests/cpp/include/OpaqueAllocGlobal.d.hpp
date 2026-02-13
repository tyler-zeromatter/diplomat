#ifndef SOMELIB_OpaqueAllocGlobal_D_HPP
#define SOMELIB_OpaqueAllocGlobal_D_HPP

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
    struct OpaqueAllocGlobal;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueAllocGlobal {
public:

    inline const somelib::capi::OpaqueAllocGlobal* AsFFI() const;
    inline somelib::capi::OpaqueAllocGlobal* AsFFI();
    inline static const somelib::OpaqueAllocGlobal* FromFFI(const somelib::capi::OpaqueAllocGlobal* ptr);
    inline static somelib::OpaqueAllocGlobal* FromFFI(somelib::capi::OpaqueAllocGlobal* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueAllocGlobal() = delete;
    OpaqueAllocGlobal(const somelib::OpaqueAllocGlobal&) = delete;
    OpaqueAllocGlobal(somelib::OpaqueAllocGlobal&&) noexcept = delete;
    OpaqueAllocGlobal operator=(const somelib::OpaqueAllocGlobal&) = delete;
    OpaqueAllocGlobal operator=(somelib::OpaqueAllocGlobal&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueAllocGlobal_D_HPP

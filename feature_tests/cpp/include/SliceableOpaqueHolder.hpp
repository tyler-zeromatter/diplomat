#ifndef SOMELIB_SliceableOpaqueHolder_HPP
#define SOMELIB_SliceableOpaqueHolder_HPP

#include "SliceableOpaqueHolder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "SliceableOpaque.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::SliceableOpaqueHolder* SliceableOpaqueHolder_new(const somelib::capi::SliceableOpaque* sl);

    somelib::capi::DiplomatSliceableOpaqueView SliceableOpaqueHolder_mismatch_lt_ret(const somelib::capi::SliceableOpaqueHolder* self);

    void SliceableOpaqueHolder_destroy(SliceableOpaqueHolder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::SliceableOpaqueHolder> somelib::SliceableOpaqueHolder::new_(const somelib::SliceableOpaque& sl) {
    auto result = somelib::capi::SliceableOpaqueHolder_new(sl.AsFFI());
    return std::unique_ptr<somelib::SliceableOpaqueHolder>(somelib::SliceableOpaqueHolder::FromFFI(result));
}

inline somelib::diplomat::span<const somelib::SliceableOpaque*> somelib::SliceableOpaqueHolder::mismatch_lt_ret() const {
    auto result = somelib::capi::SliceableOpaqueHolder_mismatch_lt_ret(this->AsFFI());
    return somelib::diplomat::span<const somelib::SliceableOpaque*>(reinterpret_cast<const somelib::SliceableOpaque**>(result.data), result.len);
}

inline const somelib::capi::SliceableOpaqueHolder* somelib::SliceableOpaqueHolder::AsFFI() const {
    return reinterpret_cast<const somelib::capi::SliceableOpaqueHolder*>(this);
}

inline somelib::capi::SliceableOpaqueHolder* somelib::SliceableOpaqueHolder::AsFFI() {
    return reinterpret_cast<somelib::capi::SliceableOpaqueHolder*>(this);
}

inline const somelib::SliceableOpaqueHolder* somelib::SliceableOpaqueHolder::FromFFI(const somelib::capi::SliceableOpaqueHolder* ptr) {
    return reinterpret_cast<const somelib::SliceableOpaqueHolder*>(ptr);
}

inline somelib::SliceableOpaqueHolder* somelib::SliceableOpaqueHolder::FromFFI(somelib::capi::SliceableOpaqueHolder* ptr) {
    return reinterpret_cast<somelib::SliceableOpaqueHolder*>(ptr);
}

inline void somelib::SliceableOpaqueHolder::operator delete(void* ptr) {
    somelib::capi::SliceableOpaqueHolder_destroy(reinterpret_cast<somelib::capi::SliceableOpaqueHolder*>(ptr));
}


#endif // SOMELIB_SliceableOpaqueHolder_HPP

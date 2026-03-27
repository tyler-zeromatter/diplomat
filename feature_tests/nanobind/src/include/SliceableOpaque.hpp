#ifndef SOMELIB_SliceableOpaque_HPP
#define SOMELIB_SliceableOpaque_HPP

#include "SliceableOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "SliceableOpaqueHolder.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::SliceableOpaque* SliceableOpaque_new(int32_t i);

    int32_t SliceableOpaque_num(const somelib::capi::SliceableOpaque* self);

    void SliceableOpaque_static_in(somelib::capi::DiplomatSliceableOpaqueView i, int32_t n);

    void SliceableOpaque_non_static_mismatch_in(somelib::capi::DiplomatSliceableOpaqueView i, int32_t n);

    somelib::capi::DiplomatSliceableOpaqueView SliceableOpaque_static_ret(void);

    somelib::capi::SliceableOpaqueHolder* SliceableOpaque_make_static_holder(void);

    void SliceableOpaque_destroy(SliceableOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::SliceableOpaque> somelib::SliceableOpaque::new_(int32_t i) {
    auto result = somelib::capi::SliceableOpaque_new(i);
    return std::unique_ptr<somelib::SliceableOpaque>(somelib::SliceableOpaque::FromFFI(result));
}

inline int32_t somelib::SliceableOpaque::num() const {
    auto result = somelib::capi::SliceableOpaque_num(this->AsFFI());
    return result;
}

inline void somelib::SliceableOpaque::static_in(somelib::diplomat::span<const somelib::SliceableOpaque*> i, int32_t n) {
    somelib::capi::SliceableOpaque_static_in({reinterpret_cast<const somelib::capi::SliceableOpaque**>(i.data()), i.size()},
        n);
}

inline void somelib::SliceableOpaque::non_static_mismatch_in(somelib::diplomat::span<const somelib::SliceableOpaque*> i, int32_t n) {
    somelib::capi::SliceableOpaque_non_static_mismatch_in({reinterpret_cast<const somelib::capi::SliceableOpaque**>(i.data()), i.size()},
        n);
}

inline somelib::diplomat::span<const somelib::SliceableOpaque*> somelib::SliceableOpaque::static_ret() {
    auto result = somelib::capi::SliceableOpaque_static_ret();
    return somelib::diplomat::span<const somelib::SliceableOpaque*>(reinterpret_cast<const somelib::SliceableOpaque**>(result.data), result.len);
}

inline std::unique_ptr<somelib::SliceableOpaqueHolder> somelib::SliceableOpaque::make_static_holder() {
    auto result = somelib::capi::SliceableOpaque_make_static_holder();
    return std::unique_ptr<somelib::SliceableOpaqueHolder>(somelib::SliceableOpaqueHolder::FromFFI(result));
}

inline const somelib::capi::SliceableOpaque* somelib::SliceableOpaque::AsFFI() const {
    return reinterpret_cast<const somelib::capi::SliceableOpaque*>(this);
}

inline somelib::capi::SliceableOpaque* somelib::SliceableOpaque::AsFFI() {
    return reinterpret_cast<somelib::capi::SliceableOpaque*>(this);
}

inline const somelib::SliceableOpaque* somelib::SliceableOpaque::FromFFI(const somelib::capi::SliceableOpaque* ptr) {
    return reinterpret_cast<const somelib::SliceableOpaque*>(ptr);
}

inline somelib::SliceableOpaque* somelib::SliceableOpaque::FromFFI(somelib::capi::SliceableOpaque* ptr) {
    return reinterpret_cast<somelib::SliceableOpaque*>(ptr);
}

inline void somelib::SliceableOpaque::operator delete(void* ptr) {
    somelib::capi::SliceableOpaque_destroy(reinterpret_cast<somelib::capi::SliceableOpaque*>(ptr));
}


#endif // SOMELIB_SliceableOpaque_HPP

#ifndef SOMELIB_SliceableOpaque_D_HPP
#define SOMELIB_SliceableOpaque_D_HPP

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
namespace capi { struct SliceableOpaque; }
class SliceableOpaque;
namespace capi { struct SliceableOpaqueHolder; }
class SliceableOpaqueHolder;
} // namespace somelib



namespace somelib {
namespace capi {
    struct SliceableOpaque;


    typedef struct DiplomatSliceableOpaqueView {
      const SliceableOpaque** data;
      size_t len;
    } DiplomatSliceableOpaqueView;

    typedef struct DiplomatSliceableOpaqueViewMut {
      SliceableOpaque** data;
      size_t len;
    } DiplomatSliceableOpaqueViewMut;
} // namespace capi
} // namespace

namespace somelib {
class SliceableOpaque {
public:

  inline static std::unique_ptr<somelib::SliceableOpaque> new_(int32_t i);

  inline int32_t num() const;

  inline static void static_in(somelib::diplomat::span<const somelib::SliceableOpaque*> i, int32_t n);

  inline static void non_static_mismatch_in(somelib::diplomat::span<const somelib::SliceableOpaque*> i, int32_t n);

  inline static somelib::diplomat::span<const somelib::SliceableOpaque*> static_ret();

  inline static std::unique_ptr<somelib::SliceableOpaqueHolder> make_static_holder();

  inline static somelib::diplomat::span<const somelib::SliceableOpaque*> optional_opaque_inout(somelib::diplomat::span<const somelib::SliceableOpaque*> sl);

    inline const somelib::capi::SliceableOpaque* AsFFI() const;
    inline somelib::capi::SliceableOpaque* AsFFI();
    inline static const somelib::SliceableOpaque* FromFFI(const somelib::capi::SliceableOpaque* ptr);
    inline static somelib::SliceableOpaque* FromFFI(somelib::capi::SliceableOpaque* ptr);
    inline static void operator delete(void* ptr);
private:
    SliceableOpaque() = delete;
    SliceableOpaque(const somelib::SliceableOpaque&) = delete;
    SliceableOpaque(somelib::SliceableOpaque&&) noexcept = delete;
    SliceableOpaque operator=(const somelib::SliceableOpaque&) = delete;
    SliceableOpaque operator=(somelib::SliceableOpaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_SliceableOpaque_D_HPP

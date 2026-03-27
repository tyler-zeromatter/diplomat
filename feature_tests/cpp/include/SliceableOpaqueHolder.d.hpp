#ifndef SOMELIB_SliceableOpaqueHolder_D_HPP
#define SOMELIB_SliceableOpaqueHolder_D_HPP

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
    struct SliceableOpaqueHolder;


    typedef struct DiplomatSliceableOpaqueHolderView {
      const SliceableOpaqueHolder** data;
      size_t len;
    } DiplomatSliceableOpaqueHolderView;

    typedef struct DiplomatSliceableOpaqueHolderViewMut {
      SliceableOpaqueHolder** data;
      size_t len;
    } DiplomatSliceableOpaqueHolderViewMut;
} // namespace capi
} // namespace

namespace somelib {
class SliceableOpaqueHolder {
public:

  inline static std::unique_ptr<somelib::SliceableOpaqueHolder> new_(const somelib::SliceableOpaque& sl);

  inline somelib::diplomat::span<const somelib::SliceableOpaque*> mismatch_lt_ret() const;

    inline const somelib::capi::SliceableOpaqueHolder* AsFFI() const;
    inline somelib::capi::SliceableOpaqueHolder* AsFFI();
    inline static const somelib::SliceableOpaqueHolder* FromFFI(const somelib::capi::SliceableOpaqueHolder* ptr);
    inline static somelib::SliceableOpaqueHolder* FromFFI(somelib::capi::SliceableOpaqueHolder* ptr);
    inline static void operator delete(void* ptr);
private:
    SliceableOpaqueHolder() = delete;
    SliceableOpaqueHolder(const somelib::SliceableOpaqueHolder&) = delete;
    SliceableOpaqueHolder(somelib::SliceableOpaqueHolder&&) noexcept = delete;
    SliceableOpaqueHolder operator=(const somelib::SliceableOpaqueHolder&) = delete;
    SliceableOpaqueHolder operator=(somelib::SliceableOpaqueHolder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_SliceableOpaqueHolder_D_HPP

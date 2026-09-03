#ifndef SOMELIB_FallibleTraitWrapper_D_HPP
#define SOMELIB_FallibleTraitWrapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FallibleTesterTrait.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
class FFIError;
} // namespace somelib



namespace somelib {
namespace capi {
    struct FallibleTraitWrapper {
      bool cant_be_empty;
    };

    typedef struct FallibleTraitWrapper_option {union { FallibleTraitWrapper ok; }; bool is_ok; } FallibleTraitWrapper_option;
} // namespace capi
} // namespace


namespace somelib {
struct FallibleTraitWrapper {
    bool cant_be_empty;

  inline static void test_with_trait(std::unique_ptr<somelib::FallibleTesterTrait> t);

  inline static somelib::diplomat::result<uint32_t, somelib::FFIError> test_result_output(std::unique_ptr<somelib::FallibleTesterTrait> t, uint32_t x);

    inline somelib::capi::FallibleTraitWrapper AsFFI() const;
    inline static somelib::FallibleTraitWrapper FromFFI(somelib::capi::FallibleTraitWrapper c_struct);
};

} // namespace
#endif // SOMELIB_FallibleTraitWrapper_D_HPP

#ifndef SOMELIB_FallibleTraitWrapper_HPP
#define SOMELIB_FallibleTraitWrapper_HPP

#include "FallibleTraitWrapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FFIError.hpp"
#include "FallibleTesterTrait.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void FallibleTraitWrapper_test_with_trait(somelib::capi::DiplomatTraitStruct_FallibleTesterTrait t_trait_wrap);

    typedef struct FallibleTraitWrapper_test_result_output_result {union {uint32_t ok; somelib::capi::FFIError err;}; bool is_ok;} FallibleTraitWrapper_test_result_output_result;
    FallibleTraitWrapper_test_result_output_result FallibleTraitWrapper_test_result_output(somelib::capi::DiplomatTraitStruct_FallibleTesterTrait t_trait_wrap, uint32_t x);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::FallibleTraitWrapper::test_with_trait(std::unique_ptr<somelib::FallibleTesterTrait> t) {
    somelib::capi::FallibleTraitWrapper_test_with_trait(t.release()->AsFFI());
}

inline somelib::diplomat::result<uint32_t, somelib::FFIError> somelib::FallibleTraitWrapper::test_result_output(std::unique_ptr<somelib::FallibleTesterTrait> t, uint32_t x) {
    auto result = somelib::capi::FallibleTraitWrapper_test_result_output(t.release()->AsFFI(),
        x);
    return result.is_ok ? somelib::diplomat::result<uint32_t, somelib::FFIError>(somelib::diplomat::Ok<uint32_t>(result.ok)) : somelib::diplomat::result<uint32_t, somelib::FFIError>(somelib::diplomat::Err<somelib::FFIError>(somelib::FFIError::FromFFI(result.err)));
}


inline somelib::capi::FallibleTraitWrapper somelib::FallibleTraitWrapper::AsFFI() const {
    return somelib::capi::FallibleTraitWrapper {
        /* .cant_be_empty = */ cant_be_empty,
    };
}

inline somelib::FallibleTraitWrapper somelib::FallibleTraitWrapper::FromFFI(somelib::capi::FallibleTraitWrapper c_struct) {
    return somelib::FallibleTraitWrapper {
        /* .cant_be_empty = */ c_struct.cant_be_empty,
    };
}


#endif // SOMELIB_FallibleTraitWrapper_HPP

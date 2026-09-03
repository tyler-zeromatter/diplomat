#ifndef SOMELIB_FallibleTesterTrait_HPP
#define SOMELIB_FallibleTesterTrait_HPP

#include "FallibleTesterTrait.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FFIError.hpp"
#include "diplomat_runtime.hpp"


inline somelib::capi::DiplomatTraitStruct_FallibleTesterTrait somelib::FallibleTesterTrait::AsFFI() const {
    struct somelib::capi::DiplomatTraitStruct_FallibleTesterTrait trait_inner = {
        (void*)this,
        {
            &somelib::FallibleTesterTrait::Destroy,
            somelib::capi::FallibleTesterTrait_DATA_SIZE,
            somelib::capi::FallibleTesterTrait_DATA_ALIGNMENT,
            [](void* self) -> somelib::capi::test_void_trait_fn_result {
                return somelib::diplomat::fn_trait_helpers::replace_result<std::monostate, somelib::FFIError, somelib::capi::test_void_trait_fn_result>(reinterpret_cast<somelib::FallibleTesterTrait*>(self)->test_void_trait_fn());
            },
            [](void* self, uint32_t x) -> somelib::capi::test_result_output_result {
                return somelib::diplomat::fn_trait_helpers::replace_result<uint32_t, somelib::FFIError, somelib::capi::test_result_output_result>(reinterpret_cast<somelib::FallibleTesterTrait*>(self)->test_result_output(somelib::diplomat::fn_trait_helpers::replace<uint32_t>(x)));
            },
        }
    };
    return trait_inner;
}
#endif // SOMELIB_FallibleTesterTrait_HPP

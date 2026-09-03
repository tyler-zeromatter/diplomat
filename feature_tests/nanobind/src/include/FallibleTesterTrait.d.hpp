#ifndef SOMELIB_FallibleTesterTrait_D_HPP
#define SOMELIB_FallibleTesterTrait_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "FFIError.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
class FFIError;
} // namespace somelib



namespace somelib {
namespace capi {
    typedef struct test_void_trait_fn_result {union { somelib::capi::FFIError err;}; bool is_ok;} test_void_trait_fn_result;

    typedef struct test_result_output_result {union {uint32_t ok; somelib::capi::FFIError err;}; bool is_ok;} test_result_output_result;
    struct FallibleTesterTrait_VTable {
        void (*destructor)(const void*);
        size_t SIZE; size_t ALIGNMENT;
        test_void_trait_fn_result (*run_test_void_trait_fn_callback)(void*);
        test_result_output_result (*run_test_result_output_callback)(void*, uint32_t);
    };

    struct DiplomatTraitStruct_FallibleTesterTrait {
        void* data;
        FallibleTesterTrait_VTable vtable;
    };

    static void general_destructor(const void* data) {
        // TODO
    }

    const size_t FallibleTesterTrait_DATA_SIZE = 0;
    const size_t FallibleTesterTrait_DATA_ALIGNMENT = 0;
} // namespace capi
} // namespace

namespace somelib {
class FallibleTesterTrait {
    private:
    static void Destroy(const void* data) {
        auto self = static_cast<const somelib::FallibleTesterTrait*>(data);
        delete self;
    }

    protected:
    virtual somelib::diplomat::result<std::monostate, somelib::FFIError> test_void_trait_fn() = 0;
    virtual somelib::diplomat::result<uint32_t, somelib::FFIError> test_result_output(uint32_t x) = 0;

    public:
    inline somelib::capi::DiplomatTraitStruct_FallibleTesterTrait AsFFI() const;
};


} // namespace
#endif // SOMELIB_FallibleTesterTrait_D_HPP

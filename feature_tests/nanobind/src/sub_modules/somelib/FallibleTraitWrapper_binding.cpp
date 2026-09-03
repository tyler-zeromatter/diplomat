#include "diplomat_nanobind_common.hpp"


#include "FallibleTesterTrait.hpp"
#include "FallibleTraitWrapper.hpp"

namespace somelib {
void add_FallibleTraitWrapper_binding(nb::module_ mod) {
    nb::class_<somelib::FallibleTraitWrapper> st(mod, "FallibleTraitWrapper");
    st
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &somelib::FallibleTraitWrapper::cant_be_empty)
        .def_static("test_result_output", &somelib::FallibleTraitWrapper::test_result_output, "t"_a, "x"_a)
        .def_static("test_with_trait", &somelib::FallibleTraitWrapper::test_with_trait, "t"_a);
}

} 
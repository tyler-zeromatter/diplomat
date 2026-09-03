#include "diplomat_nanobind_common.hpp"


#include "TraitTestingStruct.hpp"

namespace somelib {
void add_TraitTestingStruct_binding(nb::module_ mod) {
    nb::class_<somelib::TraitTestingStruct> st(mod, "TraitTestingStruct");
    st
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "x"_a.none(),  "y"_a.none())
        .def_rw("x", &somelib::TraitTestingStruct::x)
        .def_rw("y", &somelib::TraitTestingStruct::y);
}

} 
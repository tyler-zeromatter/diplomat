#include "diplomat_nanobind_common.hpp"


#include "SliceableOpaque.hpp"

namespace somelib {
void add_SliceableOpaque_binding(nb::module_ mod) {
    PyType_Slot somelib_SliceableOpaque_slots[] = {
        {Py_tp_free, (void *)somelib::SliceableOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::SliceableOpaque> opaque(mod, "SliceableOpaque", nb::type_slots(somelib_SliceableOpaque_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::SliceableOpaque::new_))), "i"_a)
        .def_static("make_static_holder", std::move(maybe_op_unwrap(&somelib::SliceableOpaque::make_static_holder)))
        .def_static("non_static_mismatch_in", &somelib::SliceableOpaque::non_static_mismatch_in, "i"_a, "n"_a)
        .def_prop_ro("num", &somelib::SliceableOpaque::num)
        .def_static("optional_opaque_inout", &somelib::SliceableOpaque::optional_opaque_inout, "sl"_a)
        .def_static("static_in", &somelib::SliceableOpaque::static_in, "i"_a, "n"_a)
        .def_static("static_ret", &somelib::SliceableOpaque::static_ret);
}

} 
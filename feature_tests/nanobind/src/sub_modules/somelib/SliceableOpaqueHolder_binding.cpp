#include "diplomat_nanobind_common.hpp"


#include "SliceableOpaque.hpp"
#include "SliceableOpaqueHolder.hpp"

namespace somelib {
void add_SliceableOpaqueHolder_binding(nb::module_ mod) {
    PyType_Slot somelib_SliceableOpaqueHolder_slots[] = {
        {Py_tp_free, (void *)somelib::SliceableOpaqueHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::SliceableOpaqueHolder> opaque(mod, "SliceableOpaqueHolder", nb::type_slots(somelib_SliceableOpaqueHolder_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::SliceableOpaqueHolder::new_))), "sl"_a, nb::keep_alive<1, 2>())
        .def("mismatch_lt_ret", &somelib::SliceableOpaqueHolder::mismatch_lt_ret);
}

} 
#include "diplomat_nanobind_common.hpp"


#include "FFIError.hpp"

namespace somelib {
void add_FFIError_binding(nb::module_ mod) {
    nb::class_<somelib::FFIError> e_class(mod, "FFIError");
    
        nb::enum_<somelib::FFIError::Value> enumerator(e_class, "FFIError");
        enumerator
            .value("FFI", somelib::FFIError::FFI)
            .value("User", somelib::FFIError::User)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::FFIError::Value>())
            .def(nb::self == somelib::FFIError::Value())
            .def("__repr__", [](const somelib::FFIError& self){
                return nb::str(nb::cast(somelib::FFIError::Value(self)));
            });
}

} 
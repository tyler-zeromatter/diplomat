#include <iostream>
#include "../include/MyString.hpp"
#include "../include/SliceableOpaque.hpp"
#include "assert.hpp"

using namespace somelib;


int main(int argc, char* argv[]) {
    auto a = MyString::new_("Test");
    auto b = MyString::new_(" String ");
    auto c = MyString::new_("end.");

    const MyString* arr[] = {
        a.get(), b.get(), c.get()
    };
    diplomat::span<const MyString*> in(arr, 3);
    simple_assert_eq("Slice of opaques", MyString::slice_of_opaques(in), "Test String end.");

    auto out = MyString::return_slice_of_opaques(in);
    simple_assert_eq("Return slice of opaques", out.data()[0]->get_str(), "Test");

    auto static_out = SliceableOpaque::static_ret();
    SliceableOpaque::static_in(static_out, 20);
}
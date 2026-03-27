#include <iostream>
#include "../include/MyString.hpp"
#include "../include/SliceableOpaque.hpp"
#include "../include/SliceableOpaqueHolder.hpp"
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

    auto op_a = SliceableOpaque::new_(0);
    auto op_b = SliceableOpaque::new_(5);
    auto op_c = SliceableOpaque::new_(10);
    const SliceableOpaque* sliceable_arr[] = {
        op_a.get(), op_b.get(), op_c.get()
    };
    diplomat::span<const SliceableOpaque*> sl_in(sliceable_arr, 3);
    SliceableOpaque::non_static_mismatch_in(sl_in, 0);

    auto static_holder = SliceableOpaque::make_static_holder();
    SliceableOpaque::non_static_mismatch_in(static_holder->mismatch_lt_ret(), 20);

    auto optional_opaque_inout = SliceableOpaque::optional_opaque_inout(sliceable_arr);
    simple_assert_eq("Optional Opaque Slice", optional_opaque_inout.data()[1]->num(), 5);
}
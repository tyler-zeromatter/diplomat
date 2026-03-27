import somelib
import somelib.somelib

import pytest

def test_slices():
    sl = somelib.Float64Vec.new([.1, .2, .3]).asSlice
    b = somelib.Float64Vec.new([.1, .2, .3]).borrow()
    assert all(sl == [.1, .2, .3])
    assert all(b == [.1, .2, .3])

    s = somelib.MyString("banannas").get_static_str()
    b = somelib.MyString("banannas").borrow()
    assert s == "hello"
    assert b == "banannas"
    assert s is not b

    c = somelib.Float64Vec.new([.1, .2, .3])
    d = somelib.Float64VecError.new([.1, .2, .3])
    
    with pytest.raises(IndexError):
        c[4]
        d[4]
    
    l = [somelib.MyString("A"), somelib.MyString(" B "), somelib.MyString("C")]
    assert somelib.MyString.slice_of_opaques(l) == "A B C"

    out = somelib.MyString.return_slice_of_opaques(l)
    assert out[0].str == l[0].str
    assert out[1].str == l[1].str

    static_out = somelib.SliceableOpaque.static_ret()
    somelib.SliceableOpaque.static_in(static_out, 20)

    sliceable_arr = [somelib.SliceableOpaque(0), somelib.SliceableOpaque(5), somelib.SliceableOpaque(10)]
    somelib.SliceableOpaque.non_static_mismatch_in(sliceable_arr, 0)
    
    static_holder = somelib.SliceableOpaque.make_static_holder()
    somelib.SliceableOpaque.non_static_mismatch_in(static_holder.mismatch_lt_ret(), 20)
    
    optional_outoptional_opaque_inout = somelib.SliceableOpaque.optional_opaque_inout([somelib.SliceableOpaque(0), None, somelib.SliceableOpaque(5)]);
    assert optional_outoptional_opaque_inout[1] == None
    assert optional_outoptional_opaque_inout[0].num == 0
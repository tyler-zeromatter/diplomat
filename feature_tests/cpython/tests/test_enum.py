import _somelib as somelib

def test_enum_variants():
    assert somelib.MyEnum.A.value == -2
    assert somelib.MyEnum.B.value == -1
    assert somelib.MyEnum.E.value == 2
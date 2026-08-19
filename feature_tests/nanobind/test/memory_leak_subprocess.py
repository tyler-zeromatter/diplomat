import somelib

a = somelib.OpaqueMutexedString.from_usize(0)
a.borrow()
b = somelib.OpaqueMutexedString.from_usize(0)
a.borrow_self_or_other(b)

f = somelib.Float64Vec.new([0, 1, 2])
f.borrow()

# Test ZST memory leaks:
a = somelib.ns.RenamedOpaqueZST()
b = somelib.ns.RenamedOpaqueZST()

a = somelib.ResultOpaque(0)
try:
    a.give_self()
except Exception as e:
    pass

try:
    somelib.ResultOpaque.new_failing_struct(109)
except Exception as e:
    pass

class ImplementsTrait(somelib.FallibleTesterTrait):
    def __init__(self):
        super().__init__()
        

    def test_void_trait_fn(self):
        pass

    def test_result_output(self, x : int):
        return x

i = ImplementsTrait()
assert somelib.FallibleTraitWrapper.test_result_output(i, 5) == 5
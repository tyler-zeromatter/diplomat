pub struct RenamedVectorTest;

impl RenamedVectorTest {
    fn new() -> RenamedVectorTest {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_new() }
    }

    fn len(&self) -> usize {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_len(self) }
    }

    fn get(&self, idx : usize) -> Option<f64> {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_get(self, idx) }
    }

    fn push(&mut self, value : f64) {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_push(self, value) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_VectorTest_new() -> RenamedVectorTest;

    fn namespace_VectorTest_len(this: &RenamedVectorTest) -> usize;

    fn namespace_VectorTest_get(this: &RenamedVectorTest, idx : usize) -> Option<f64>;

    fn namespace_VectorTest_push(this: &mut RenamedVectorTest, value : f64);

}
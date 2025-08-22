pub struct RenamedVectorTest;

impl RenamedVectorTest {
    pub fn new() -> Box<RenamedVectorTest> {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_new() }
    }

    pub fn len(&self) -> usize {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_len(self) }
    }

    pub fn get(&self, idx : usize) -> Option<f64> {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_get(self, idx) }
    }

    pub fn push(&mut self, value : f64) {
            // TODO: writeable conversions.
        unsafe { namespace_VectorTest_push(self, value) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_VectorTest_new() -> Box<RenamedVectorTest>;

    fn namespace_VectorTest_len(this: &RenamedVectorTest) -> usize;

    fn namespace_VectorTest_get(this: &RenamedVectorTest, idx : usize) -> DiplomatOption<f64>;

    fn namespace_VectorTest_push(this: &mut RenamedVectorTest, value : f64);

}
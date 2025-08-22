pub struct RenamedVectorTest;

impl Drop for RenamedVectorTest {
    fn drop(&mut self) {
        unsafe { RenamedVectorTest_destroy(self) }
    }
}

impl RenamedVectorTest {
    pub fn new() -> Box<RenamedVectorTest> {
        let ret = unsafe { namespace_VectorTest_new() };
        ret
    }

    pub fn len(&self) -> usize {
        let ret = unsafe { namespace_VectorTest_len(self) };
        ret
    }

    pub fn get(&self, idx : usize) -> Option<f64> {
        let ret = unsafe { namespace_VectorTest_get(self, idx) };
        ret.into_converted_option()
    }

    pub fn push(&mut self, value : f64) {
        let ret = unsafe { namespace_VectorTest_push(self, value) };
        ret
    }

    

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn namespace_VectorTest_new() -> Box<RenamedVectorTest>;

    fn namespace_VectorTest_len(this: &RenamedVectorTest) -> usize;

    fn namespace_VectorTest_get(this: &RenamedVectorTest, idx : usize) -> diplomat_runtime::DiplomatOption<f64>;

    fn namespace_VectorTest_push(this: &mut RenamedVectorTest, value : f64);

    fn RenamedVectorTest_destroy(this : *mut RenamedVectorTest);

}
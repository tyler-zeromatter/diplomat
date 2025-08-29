pub struct RenamedVectorTest;

impl Drop for RenamedVectorTest {
    fn drop(&mut self) {
        unsafe { namespace_VectorTest_destroy(self) }
    }
}

impl RenamedVectorTest {
    pub fn new() -> Box<RenamedVectorTest> {
        let ret = unsafe { namespace_VectorTest_new() };
        
        ret
    
    }

    pub fn len<'anon_0>(&'anon_0 self) -> usize {
        let ret = unsafe { namespace_VectorTest_len(self) };
        
        ret
    
    }

    pub fn get<'anon_0>(&'anon_0 self, idx : usize) -> Option<f64> {
        let ret = unsafe { namespace_VectorTest_get(self, idx) };
        
        ret.into_converted_option()
    
    }

    pub fn push<'anon_0>(&'anon_0 mut self, value : f64) {
        unsafe { namespace_VectorTest_push(self, value) };
        
    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn namespace_VectorTest_new() -> Box<RenamedVectorTest>;

    fn namespace_VectorTest_len<'anon_0>(this: &'anon_0 RenamedVectorTest) -> usize;

    fn namespace_VectorTest_get<'anon_0>(this: &'anon_0 RenamedVectorTest, idx : usize) -> diplomat_runtime::DiplomatOption<f64>;

    fn namespace_VectorTest_push<'anon_0>(this: &'anon_0 mut RenamedVectorTest, value : f64);

    fn namespace_VectorTest_destroy(this : *mut RenamedVectorTest);
}
use super::OpaqueThin;
pub struct OpaqueThinIter;

impl OpaqueThinIter {
    pub fn next(&mut self) -> &Option<OpaqueThin> {
            // TODO: writeable conversions.
        unsafe { OpaqueThinIter_next(self) }
    }

}

#[link(name = "somelib")]
unsafe extern "C" {
    fn OpaqueThinIter_next(this: &mut OpaqueThinIter) -> &Option<OpaqueThin>;

}
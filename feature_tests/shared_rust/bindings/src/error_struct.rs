#[repr(C)]
pub struct ErrorStruct {
    pub i: i32,
    pub j: i32,
}

impl ErrorStruct {
    pub fn returns_result_option(is_some : bool) -> Result<Option<ErrorStruct>, ()> {
        let ret = unsafe { ErrorStruct_returns_result_option(is_some) };
        
        ret.to_result().map(|ok : diplomat_runtime::DiplomatOption::<ErrorStruct>| { ok.into_converted_option() })

    }
}

#[link(name = "somelib")]
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn ErrorStruct_returns_result_option(is_some : bool) -> crate::DiplomatResult<diplomat_runtime::DiplomatOption::<ErrorStruct>, ()>;
}
#[repr(C)]
pub struct MyZst {

}

impl MyZst {
}

#[link(name = "somelib")]
extern "C" {
}
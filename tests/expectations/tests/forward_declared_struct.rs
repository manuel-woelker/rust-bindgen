/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_a() {
    assert_eq!(::std::mem::size_of::<a>() , 4usize);
    assert_eq!(::std::mem::align_of::<a>() , 4usize);
}
impl Clone for a {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_c() {
    assert_eq!(::std::mem::size_of::<c>() , 4usize);
    assert_eq!(::std::mem::align_of::<c>() , 4usize);
}
impl Clone for c {
    fn clone(&self) -> Self { *self }
}

/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo__bindgen_ty_1 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 8usize);
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . x as * const _ as
                usize } , 0usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . y as * const _ as
                usize } , 4usize);
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 8usize);
    assert_eq! (::std::mem::align_of::<foo>() , 4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . bar as * const _ as usize } ,
                0usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}

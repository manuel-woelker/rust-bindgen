/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo {
    pub bar: [foo__bindgen_ty_1; 2usize],
    pub baz: [[[foo__bindgen_ty_2; 4usize]; 3usize]; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 8usize);
    assert_eq!(::std::mem::align_of::<foo__bindgen_ty_1>() , 4usize);
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo__bindgen_ty_2 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_2>() , 8usize);
    assert_eq!(::std::mem::align_of::<foo__bindgen_ty_2>() , 4usize);
}
impl Clone for foo__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 208usize);
    assert_eq!(::std::mem::align_of::<foo>() , 4usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}

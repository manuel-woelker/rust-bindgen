/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub const FOO_BAR: _bindgen_ty_1 = _bindgen_ty_1::FOO_BAR;
pub const FOO_BAZ: _bindgen_ty_1 = _bindgen_ty_1::FOO_BAZ;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 { FOO_BAR = 0, FOO_BAZ = 1, }
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_FOO_BAR: Foo__bindgen_ty_1 = Foo__bindgen_ty_1::FOO_BAR;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Foo__bindgen_ty_1 { FOO_BAR = 10, }
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize);
    assert_eq!(::std::mem::align_of::<Foo>() , 1usize);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}

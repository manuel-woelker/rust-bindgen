/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A {
    pub foo: usize,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 8usize);
    assert_eq!(::std::mem::align_of::<A>() , 8usize);
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}

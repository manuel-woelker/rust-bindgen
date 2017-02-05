/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct UnknownUnits {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_UnknownUnits() {
    assert_eq!(::std::mem::size_of::<UnknownUnits>() , 1usize);
    assert_eq!(::std::mem::align_of::<UnknownUnits>() , 1usize);
}
impl Clone for UnknownUnits {
    fn clone(&self) -> Self { *self }
}
pub type Float = f32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointTyped<units, F> {
    pub x: F,
    pub y: F,
    pub _phantom_0: ::std::marker::PhantomData<units>,
}
impl <units, F> Default for PointTyped<units, F> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type IntPoint = PointTyped<UnknownUnits, f32>;

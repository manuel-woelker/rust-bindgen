/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkedList<T> {
    pub next: *mut LinkedList<T>,
    pub prev: *mut LinkedList<T>,
}
impl <T> Default for LinkedList<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct InstantiateIt {
    pub m_list: LinkedList<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_InstantiateIt() {
    assert_eq!(::std::mem::size_of::<InstantiateIt>() , 16usize);
    assert_eq! (::std::mem::align_of::<InstantiateIt>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const InstantiateIt ) ) . m_list as * const _
                as usize } , 0usize);
}
impl Clone for InstantiateIt {
    fn clone(&self) -> Self { *self }
}
impl Default for InstantiateIt {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

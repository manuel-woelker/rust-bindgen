/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub __bindgen_anon_1: __BindgenUnionField<foo__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo__bindgen_ty_1 {
    pub _bitfield_1: u32,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 4usize);
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 4usize);
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
impl foo__bindgen_ty_1 {
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (127usize as u32)) >>
                                       0u32) as u32)
        }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_int) {
        self._bitfield_1 &= !(127usize as u32);
        self._bitfield_1 |= ((val as u32 as u32) << 0u32) & (127usize as u32);
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 &
                                        (4294967168usize as u32)) >> 7u32) as
                                      u32)
        }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_int) {
        self._bitfield_1 &= !(4294967168usize as u32);
        self._bitfield_1 |=
            ((val as u32 as u32) << 7u32) & (4294967168usize as u32);
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 4usize);
    assert_eq! (::std::mem::align_of::<foo>() , 4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . a as * const _ as usize } ,
                0usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}

/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        /// <div rustbindgen replaces="foo::Bar"></div>
        #[repr(C)]
        #[derive(Debug, Default, Copy)]
        pub struct Bar {
            pub bazz: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(::std::mem::size_of::<Bar>() , 4usize);
            assert_eq! (::std::mem::align_of::<Bar>() , 4usize);
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Bar ) ) . bazz as * const _ as
                        usize } , 0usize);
        }
        impl Clone for Bar {
            fn clone(&self) -> Self { *self }
        }
    }
    pub type ReferencesBar = root::foo::Bar;
}

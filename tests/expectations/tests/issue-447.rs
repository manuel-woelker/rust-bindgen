/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod mozilla {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod detail {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Default, Copy)]
            pub struct GuardObjectNotifier {
                pub _address: u8,
            }
            #[test]
            fn bindgen_test_layout_GuardObjectNotifier() {
                assert_eq!(::std::mem::size_of::<GuardObjectNotifier>() ,
                           1usize);
                assert_eq!(::std::mem::align_of::<GuardObjectNotifier>() ,
                           1usize);
            }
            impl Clone for GuardObjectNotifier {
                fn clone(&self) -> Self { *self }
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct JSAutoCompartment {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_JSAutoCompartment() {
        assert_eq!(::std::mem::size_of::<JSAutoCompartment>() , 1usize);
        assert_eq!(::std::mem::align_of::<JSAutoCompartment>() , 1usize);
    }
    extern "C" {
        #[link_name =
              "_ZN17JSAutoCompartmentC1EN7mozilla6detail19GuardObjectNotifierE"]
        pub fn JSAutoCompartment_JSAutoCompartment(this:
                                                       *mut root::JSAutoCompartment,
                                                   arg1:
                                                       root::mozilla::detail::GuardObjectNotifier);
    }
    impl Clone for JSAutoCompartment {
        fn clone(&self) -> Self { *self }
    }
    impl JSAutoCompartment {
        #[inline]
        pub unsafe fn new(arg1: root::mozilla::detail::GuardObjectNotifier)
         -> Self {
            let mut __bindgen_tmp = ::std::mem::uninitialized();
            JSAutoCompartment_JSAutoCompartment(&mut __bindgen_tmp, arg1);
            __bindgen_tmp
        }
    }
}

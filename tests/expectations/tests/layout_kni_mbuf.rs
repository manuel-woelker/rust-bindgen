/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub const RTE_CACHE_LINE_MIN_SIZE: ::std::os::raw::c_uint = 64;
pub const RTE_CACHE_LINE_SIZE: ::std::os::raw::c_uint = 64;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct rte_kni_mbuf {
    pub buf_addr: *mut ::std::os::raw::c_void,
    pub buf_physaddr: u64,
    pub pad0: [::std::os::raw::c_char; 2usize],
    /**< Start address of data in segment buffer. */
    pub data_off: u16,
    pub pad1: [::std::os::raw::c_char; 2usize],
    /**< Number of segments. */
    pub nb_segs: u8,
    pub pad4: [::std::os::raw::c_char; 1usize],
    /**< Offload features. */
    pub ol_flags: u64,
    pub pad2: [::std::os::raw::c_char; 4usize],
    /**< Total pkt len: sum of all segment data_len. */
    pub pkt_len: u32,
    /**< Amount of data in segment buffer. */
    pub data_len: u16,
    pub __bindgen_padding_0: [u8; 22usize],
    pub pad3: [::std::os::raw::c_char; 8usize],
    pub pool: *mut ::std::os::raw::c_void,
    pub next: *mut ::std::os::raw::c_void,
    pub __bindgen_padding_1: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_rte_kni_mbuf() {
    assert_eq!(::std::mem::size_of::<rte_kni_mbuf>() , 128usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . buf_addr as * const _
                as usize } , 0usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . buf_physaddr as *
                const _ as usize } , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pad0 as * const _ as
                usize } , 16usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . data_off as * const _
                as usize } , 18usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pad1 as * const _ as
                usize } , 20usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . nb_segs as * const _
                as usize } , 22usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pad4 as * const _ as
                usize } , 23usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . ol_flags as * const _
                as usize } , 24usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pad2 as * const _ as
                usize } , 32usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pkt_len as * const _
                as usize } , 36usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . data_len as * const _
                as usize } , 40usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pad3 as * const _ as
                usize } , 64usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . pool as * const _ as
                usize } , 72usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const rte_kni_mbuf ) ) . next as * const _ as
                usize } , 80usize);
}
impl Clone for rte_kni_mbuf {
    fn clone(&self) -> Self { *self }
}

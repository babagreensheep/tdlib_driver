#![allow(dead_code, non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod td_api;
include!(concat!(env!("OUT_DIR"), "/td_api.rs"));

#[test]
fn client() {
    let client = unsafe { td_create_client_id() };
    assert_eq!(client, 1);
}

#![cfg(windows)]
#![allow(non_snake_case)]

use minidl::*;
use std::assert_eq;
use std::fmt::{self, Debug, Formatter};
use std::io::Result;
use std::os::raw::*;

type PassThruDisconnectFn = unsafe extern "stdcall" fn(channel_id: c_ulong) -> c_long;

#[test] fn ok_ord() {
    unsafe {
        let PassThruDisconnect : PassThruDisconnectFn = Library::load("VI2ApplicationDriver.dll").unwrap().ord(0x04).unwrap();  // PassThruDisconnect
        let channel_id = 0;
        let ret_val = PassThruDisconnect(channel_id);
        assert_eq!(ret_val, 1);
    }
}

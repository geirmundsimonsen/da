#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub struct asd {
    pub h: opds,
    pub _out: *mut f64,
    pub _in: *mut f64,
    pub _in2: *mut f64,
}

fn asd_scalar(csound: *mut CSOUND, p: *mut asd) -> i32 {
    *p._out = 2.0;

    return 0;
}
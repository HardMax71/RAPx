#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

/// SOUND: the returned `&'a str` is re-borrowed from `x: &'a str` — the same
/// region, so the source region does outlive the return region.
#[rapx::verify]
pub unsafe fn return_longer<'a: 'b, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    let px = x as *const str;
    unsafe { &*px }
}

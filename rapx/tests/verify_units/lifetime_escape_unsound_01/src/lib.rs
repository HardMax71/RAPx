#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

/// UNSOUND: the returned `&'a str` is actually the re-borrowed `y: &'b str`,
/// which is only valid for the shorter region `'b` (`'a: 'b`). The raw-pointer
/// round-trip (`y as *const str` then `&*py`) erases the `'b` lifetime, so the
/// escape analysis rejects it by comparing the returned region (`'a`) against
/// the source reference's region (`'b`): `'b: 'a` does not hold.
#[rapx::verify]
pub unsafe fn return_shorter_as_longer<'a: 'b, 'b>(_x: &'a str, y: &'b str) -> &'a str {
    let py = y as *const str;
    unsafe { &*py }
}

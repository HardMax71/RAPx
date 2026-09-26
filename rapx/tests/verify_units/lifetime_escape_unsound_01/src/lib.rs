#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

/// UNSOUND: the returned `&'a str` is actually the re-borrowed `y: &'b str`,
/// which is only valid for the shorter region `'b` (`'a: 'b`). The raw-pointer
/// round-trip (`y as *const str` then `&*py`) erases the `'b` lifetime, and type
/// inference relabels the re-borrowed view as `&'a str` to match the return type.
/// The lifetime ordering `'b < 'a` is therefore never checked: the escape
/// analysis only compares types / dataflow, and the `Alive` check is
/// region-insensitive (`alive_assumed ∧ !dead`).
#[rapx::verify]
pub unsafe fn return_shorter_as_longer<'a: 'b, 'b>(_x: &'a str, y: &'b str) -> &'a str {
    let py = y as *const str;
    unsafe { &*py }
}

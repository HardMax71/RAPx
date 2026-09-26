#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(unused)]

#[rapx::verify]
#[rapx::requires(ValidPtr(ptr, i32, 1))]
#[rapx::requires(Align(ptr, i32))]
#[rapx::requires(Init(ptr, i32, 1))]
#[rapx::requires(Alive(ptr, 'b))]
#[rapx::requires(Alias(ptr))]
pub unsafe fn as_ref_mut<'b>(ptr: *mut i32) -> &'b mut i32 {
    unsafe { &mut *ptr }
}

// SOUND: `ptr` is `Alive(ptr, 'a)` with `'a: 'b` (`'a` outlives `'b`), so the
// longer assumption covers the shorter demand — a lifetime narrowing.
#[rapx::verify]
#[rapx::requires(ValidPtr(ptr, i32, 1))]
#[rapx::requires(Align(ptr, i32))]
#[rapx::requires(Init(ptr, i32, 1))]
#[rapx::requires(Alive(ptr, 'a))]
#[rapx::requires(Alias(ptr))]
pub unsafe fn narrow<'a: 'b, 'b>(ptr: *mut i32) -> &'b mut i32 {
    let r = unsafe { as_ref_mut(ptr) };
    r
}

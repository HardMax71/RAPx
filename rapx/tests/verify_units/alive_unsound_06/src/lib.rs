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

// UNSOUND: `ptr` is only `Alive(ptr, 'a)`, but `as_ref_mut` requires
// `Alive(ptr, 'b)` with `'b: 'a` (`'b` outlives `'a`, the longer region) — a
// lifetime widening, not a use-after-free.
#[rapx::verify]
#[rapx::requires(ValidPtr(ptr, i32, 1))]
#[rapx::requires(Align(ptr, i32))]
#[rapx::requires(Init(ptr, i32, 1))]
#[rapx::requires(Alive(ptr, 'a))]
#[rapx::requires(Alias(ptr))]
pub unsafe fn widen<'a, 'b: 'a>(ptr: *mut i32) -> &'b mut i32 {
    let r = unsafe { as_ref_mut(ptr) };
    r
}

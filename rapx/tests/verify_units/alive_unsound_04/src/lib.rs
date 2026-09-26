#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(unused)]

#[rapx::invariant(NonNull(ptr))]
#[rapx::invariant(ValidPtr(ptr, i32, 1))]
#[rapx::invariant(Align(ptr, i32))]
#[rapx::invariant(Init(ptr, i32, 1))]
pub struct Holder {
    ptr: *mut i32,
}

#[rapx::verify]
#[rapx::requires(ValidPtr(ptr, i32, 1))]
#[rapx::requires(Align(ptr, i32))]
#[rapx::requires(Init(ptr, i32, 1))]
#[rapx::requires(Alive(ptr, 'a))]
#[rapx::requires(Alias(ptr))]
pub unsafe fn as_ref_mut<'a>(ptr: *mut i32) -> &'a mut i32 {
    unsafe { &mut *ptr }
}

// UNSOUND: `Holder.ptr` carries no `Alive` invariant, so `Alive(h.ptr)` cannot
// be proved (a raw pointer has no liveness guarantee).
#[rapx::verify]
pub fn use_after_free(h: &Holder) -> i32 {
    let r = unsafe { as_ref_mut(h.ptr) };
    *r
}

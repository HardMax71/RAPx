#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

struct Node {
    next: *mut Node,
}

impl Node {
    /// UNSOUND: `next` is an unannotated raw-pointer field, so the deref cannot
    /// discharge its `Ptr2Ref` obligations — `NonNull`/`Allocated`/`InBound`/
    /// `Align`/`Init` must all come from explicit `#[rapx::invariant]`s, and the
    /// `&self` → `&Node` view still trips the `Alias` hazard.
    #[rapx::verify]
    pub unsafe fn next_ref(&self) -> &Node {
        unsafe { &*self.next }
    }
}

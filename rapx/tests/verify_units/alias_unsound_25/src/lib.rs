#![feature(register_tool)]
#![register_tool(rapx)]
#![allow(dead_code)]

#[rapx::invariant(NonNull(next))]
#[rapx::invariant(Allocated(next, Node, 1))]
#[rapx::invariant(InBound(next, Node, 1))]
#[rapx::invariant(Align(next, Node))]
#[rapx::invariant(Init(next, Node, 1))]
struct Node {
    next: *mut Node,
}

impl Node {
    /// UNSOUND: `&self` produces `&mut` through a private raw field; calling it
    /// twice yields two `&mut` aliases to the same pointee. The memory-shape
    /// obligations (`NonNull`/`Allocated`/`InBound`/`Align`/`Init`) of the
    /// `Ptr2Ref` deref are discharged by the struct invariants, leaving only the
    /// `Alias` hazard unproved.
    #[rapx::verify]
    pub unsafe fn next_mut(&self) -> &mut Node {
        unsafe { &mut *self.next }
    }
}

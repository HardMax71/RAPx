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
    /// UNSOUND: a safe free function exposes the private raw field (through a
    /// non-first parameter), letting external code write through it, aliasing
    /// the `&Node` this method returns. The memory-shape obligations
    /// (`NonNull`/`Allocated`/`InBound`/`Align`/`Init`) of the `Ptr2Ref` deref
    /// are discharged by the struct invariants, leaving only the `Alias` hazard
    /// unproved.
    #[rapx::verify]
    pub unsafe fn get_next(&self) -> &Node {
        unsafe { &*self.next }
    }
}

fn expose_next(_tag: i32, node: &Node) -> *mut Node {
    node.next
}

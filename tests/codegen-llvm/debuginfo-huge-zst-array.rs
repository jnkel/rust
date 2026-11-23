// Ensure we generate validd debug info for a max-sized ZST array.

//@ compile-flags: -Copt-level=1 -Cdebuginfo=full -Ccodegen-units=2

fn main() {
    let _a: [(); { usize::MAX - 1 }] = [(); _];
}

// CHECK-NOT: DISubrange(count: -2
// CHECK: DISubrange(count: 18446744073709551614

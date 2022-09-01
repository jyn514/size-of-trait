# size-of-trait-impl

Tiny little crate to determine how large an unnameable type is.

## What does it look like?

```rust
use size_of_trait::size_of;

const A: usize = size_of!(f());
const B: usize = size_of!(0_u8);

fn main() {
    assert_eq!(A, 2);
    assert_eq!(B, 1);
}

async fn f() {
    let x = 1;
    std::future::ready(()).await;
    let y = 2;
}
```

## Why not use `std::mem::size_of_val`?

- `size_of_val` can't be used in most `const` contexts, since futures can't be constructed at compile time.
- `size_of_val` requires you to have a value; you have to create a future you never poll.

```rust,compile_fail,E0015,E0493
#![feature(const_size_of_val)]
async fn foo() {} // error: cannot call non-const fn `foo` in constants
const SIZE: usize = std::mem::size_of_val(&foo()); // error: constants cannot evaluate destructors
```

`size_of!` does not evaluate its arguments at all, and can be used in a const context.

## MSRV

1.54 (for `doc = include_str!`). This can be easily lowered to 1.31 (for `const fn`) if someone finds it useful.

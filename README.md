# size-of-trait-impl

Tiny little crate to determine how large an unnameable type is.

## What does it look like?

```rust
const C: usize = size_of_future!(f());
const D: usize = size_of_trait_impl!(0_u8, std::convert::Into<u32>);
const E: usize = size_of_trait_impl!((), for<'a> Foo<'a>);

fn main() {
    println!("{} {} {} {}", C, D, E, F);
}

trait Foo<'a> {}
impl<'a> Foo<'a> for () {}
```

You can add your own 'helper' macros which specialize to a specific trait, the same way `size_of_future!` works. Popular helpers may be upstreamed to this crate.

## Why not use `std::mem::size_of_val`?

- `size_of_val` can't be used in most `const` contexts, since futures can't be constructed at compile time.
- `size_of_val` requires you to have a value; you have to create a future you never poll.

`size_of_trait_impl!` does not evaluate its arguments at all, and can be used in a const context.

## MSRV

Currently, nightly is required (for `feature(type_alias_impl_trait)`).

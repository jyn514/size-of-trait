#![allow(dead_code, unused_variables)]

extern crate alloc;
use core::ops::Generator;

trait Foo<'a, 'b> {}
impl<'a> Foo<'a, 'static> for () {}
impl<'a, 'b> Foo<'a, 'b> for usize {}

fn g() -> impl for<'a> Foo<'a, 'static> {}

async fn f() {
    let x = 1;
    core::future::ready(()).await;
    let y = 2;
}

fn h() -> impl Generator<Return = u32> {
    || {
        let a = alloc::vec![0];
        let b = alloc::string::String::new();
        yield a;
        0
    }
}

#[test]
fn new_api() {
    const C: usize = size_of!(f());
    const D: usize = size_of!(0_u8);
    const E: usize = size_of!(());
    const F: usize = size_of!(0_usize);
    const G: usize = size_of!(h());

    assert_eq!(C, 2);
    assert_eq!(D, 1);
    assert_eq!(E, 0);
    assert_eq!(F, 8);
    assert_eq!(G, 32);
}

#[test]
#[allow(deprecated)]
fn original_api() {
    const C: usize = size_of_future!(f());
    const D: usize = size_of_trait_impl!(0_u8, std::convert::Into<u32>);
    const E: usize = size_of_trait_impl!((), for<'a> Foo<'a, 'static>);
    const F: usize = size_of_trait_impl!(0_usize, for<'a, 'b> Foo<'a, 'b>);
    const G: usize = size_of_trait_impl!(h(), Generator);

    assert_eq!(C, 2);
    assert_eq!(D, 1);
    assert_eq!(E, 0);
    assert_eq!(F, 8);
    assert_eq!(G, 32);
}

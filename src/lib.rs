#![feature(type_alias_impl_trait, allow_internal_unstable)]

#[macro_export]
/// Given an expression, and the name of the trait it implements, return the size of its type.
///
/// The expression will not be evaluated. This macro can be used in `const` contexts.
#[allow_internal_unstable(type_alias_impl_trait)]
macro_rules! size_of_trait_impl {
    ($impl_trait: expr, for< $($lt : lifetime),+ > $trait_name: path) => {{
        type MyImpl = impl for< $( $lt ),+ > $trait_name;
        fn __f() -> MyImpl { $impl_trait }
        std::mem::size_of::<MyImpl>()
    }};
    ($impl_trait: expr, $trait_name: path) => {{
        type MyImpl = impl $trait_name;
        fn __f() -> MyImpl { $impl_trait }
        std::mem::size_of::<MyImpl>()
    }};
}

#[macro_export]
/// Given a future, return the size of its type.
///
/// The future will not be evaluated. This macro can be used in `const` contexts.
///
/// # Example
/// ```
/// use size_of_trait::size_of_future;
/// async fn f() {
///     let x = 1;
///     std::future::ready(()).await;
/// }
/// assert_eq!(size_of_future!(f()), 2);
/// ```
macro_rules! size_of_future {
    ($fut: expr) => { $crate::size_of_trait_impl!($fut, ::core::future::Future) }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(dead_code, unused_variables)]
    fn it_works() {
        trait Foo<'a, 'b> {}
        impl<'a> Foo<'a, 'static> for () {}
        impl<'a, 'b> Foo<'a, 'b> for usize {}

        fn g() -> impl for<'a> Foo<'a, 'static> {
            loop {}
        }

        async fn f() {
            let x = 1;
            std::future::ready(()).await;
            let y = 2;
        }

        const C: usize = size_of_future!(f());
        const D: usize = size_of_trait_impl!(0_u8, std::convert::Into<u32>);
        const E: usize = size_of_trait_impl!((), for<'a> Foo<'a, 'static>);
        const F: usize = size_of_trait_impl!(0_usize, for<'a, 'b> Foo<'a, 'b>);

        assert_eq!(C, 2);
        assert_eq!(D, 1);
        assert_eq!(E, 0);
        assert_eq!(F, 8);
    }
}

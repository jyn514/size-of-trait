#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(test, feature(generators, generator_trait))]

/// Given an expression, return the size of its type.
///
/// The expression will not be evaluated. This macro can be used in `const` contexts.
///
/// # Example
/// ```
/// use size_of_trait::size_of;
/// async fn f() {
///     let x = 1;
///     std::future::ready(()).await;
/// }
/// const SIZE: usize = size_of!(f());
/// assert_eq!(SIZE, 2);
/// ```
#[macro_export]
macro_rules! size_of {
    ($f: expr) => {
        $crate::private::helper(if true {
            []
        } else {
            loop {}
            #[allow(unreachable_code)] {
                [|| [$f; 0]; 0]
            }
        })
    };
}

#[doc(hidden)]
pub mod private {
    use core::marker::PhantomData;

    #[doc(hidden)]
    pub const fn helper<T> (_: [impl FnOnce() -> [T; 0]; 0]) -> usize {
        ::core::mem::size_of::<T>()
    }

    #[doc(hidden)]
    pub struct Ty<F: FnOnce() -> R, R>(pub F, pub PhantomData<R>);

    #[doc(hidden)]
    pub const fn size<R>(_: PhantomData<R>) -> usize {
        core::mem::size_of::<R>()
    }
}

#[macro_export]
#[deprecated = "use `size_of` instead"]
#[doc(hidden)]
macro_rules! size_of_future {
    ($fut: expr) => {
        $crate::size_of_trait_impl!($fut, ::core::future::Future)
    };
}

#[macro_export]
#[deprecated = "use `size_of` instead"]
#[doc(hidden)]
macro_rules! size_of_trait_impl {
    ($impl_trait: expr, for< $($lt : lifetime),+ > $trait_name: path) => {{
        $crate::size_of!($impl_trait)
    }};
    ($impl_trait: expr, $trait_name: path) => {{
        $crate::size_of!($impl_trait)
    }};
}

#[cfg(test)]
mod tests;

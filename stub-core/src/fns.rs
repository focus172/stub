use crate::Stub;

/// Also Implemented for funtions of up to 12 argumetns that return nothing
/// or a [`Stub`].
impl Stub for fn() {
    #[inline]
    fn stub() -> Self {
        || {}
    }
}

/// Also Implemented for funtions of up to 12 argumetns that return nothing
/// or a [`Stub`].
impl<R: Stub> Stub for fn() -> R {
    #[inline]
    fn stub() -> Self {
        R::stub
    }
}

macro_rules! fn_impls {
    // Stopping criteria (1-ary tuple)
    ($T:ident) => {
        fn_impls!(@impl $T);
    };

    // Running criteria (n-ary tuple, with n >= 2)
    ($A1:ident $( $A:ident )+) => {
        fn_impls!($( $A )+);
        fn_impls!(@impl $A1 $( $A )+);
    };

    (@impl $( $T:ident )+) => {
        #[doc(hidden)]
        impl<$($T),+> Stub for fn($($T,)+) {
            fn stub() -> Self {
                #![allow(non_snake_case, unused_variables)]
                |$($T),+| {}
            }
        }

        #[doc(hidden)]
        impl<R: Stub, $($T),+> Stub for fn($($T,)+) -> R {
            fn stub() -> Self {
                #![allow(non_snake_case, unused_variables)]
                |$($T),+| { R::stub() }
            }
        }
    }
}

fn_impls!(A B C D E F G H I J K L);

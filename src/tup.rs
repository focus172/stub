//! Impliments the stub trait for tuples upt to 12 elements

use crate::Stub;

macro_rules! tuple_impl {
    ($T:ident) => {};

    ($T:ident $( $U:ident )+) => {
        tuple_impl!($( $U )+);
        tuple_impl!(@impl $T $( $U )+);
    };

    (@impl $( $T:ident )+) => {
        #[doc(hidden)]
        impl<$($T: Stub),+> Stub for ($($T),+) {
            fn stub() -> Self {
                ( $( $T::stub() ),+ )
            }
        }
    }
}

tuple_impl!(A B C D E F G H I J K L);

// impl<T: Stub, const N: usize> Stub for [T; N] {
//     fn stub() -> Self {
//         let mut data: [MaybeUninit<T>; N] = MaybeUninit::uninit().transpose();
//         for e in data.iter_mut() {
//             e.write(T::stub());
//         }
//
//         unsafe { data.transpose().assume_init() }
//     }
// }

macro_rules! array_impl_stub {
    ($n:expr, $t:ident $($ts:ident)*) => {
        #[doc(hidden)]
        impl<T: Stub> Stub for [T; $n] {
            fn stub() -> Self {
                [$t::stub(), $($ts::stub()),*]
            }
        }

        array_impl_stub!(($n - 1), $($ts)*);
    };

    // when there are no Ts left then it is the empty array
    ($n:expr,) => {
        /// Also implemented for sized arrays up to 32 elemetns
        impl<T> Stub for [T; $n] {
            fn stub() -> Self { [] }
        }
    };
}

// there are 32 Ts here
array_impl_stub!(32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T);

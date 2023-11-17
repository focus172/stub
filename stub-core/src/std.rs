#![no_implicit_prelude]

extern crate std;

use crate::Stub;


macro_rules! stub_impl {
    ($val:expr => $($t:ty)*) => {$(
        impl Stub for $t {
            /// Alias to the standard library default implimentation
            #[inline(always)]
            fn stub() -> Self {
                $val
            }
        }
    )*}
}

// stub_impl!(Default::default() => std::alloc::Global);

// This is a list of all (stable) rust types that implement Default
stub_impl!(
std::default::Default::default() =>
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
    bool
    &str char
    std::boxed::Box<str>
    std::boxed::Box<std::ffi::CStr>
    std::boxed::Box<std::ffi::OsStr>
    std::alloc::System
    std::collections::hash_map::DefaultHasher
    std::collections::hash_map::RandomState
    std::ffi::CString
    std::ffi::OsString
    std::fmt::Error
    // std::fs::FileTimes
    // std::hash::SipHasher
    std::io::Empty
    std::io::Sink
    std::marker::PhantomPinned
    std::ops::RangeFull
    std::path::PathBuf
    std::process::ExitStatus
    std::string::String
    std::sync::atomic::AtomicBool
    std::sync::atomic::AtomicI8
    std::sync::atomic::AtomicI32
    std::sync::atomic::AtomicI64
    std::sync::atomic::AtomicIsize
    std::sync::atomic::AtomicU8
    std::sync::atomic::AtomicU16
    std::sync::atomic::AtomicU32
    std::sync::atomic::AtomicU64
    std::sync::atomic::AtomicUsize
    std::sync::Condvar
    std::time::Duration
);

// things I was too lazy to add in
// will get added as needed

// impl<A, B> Default for Chain<A, B>where
//     A: Default,
//     B: Default,
// 1.11.0 · source
// impl<B> Default for Cow<'_, B>where
//     B: ToOwned + ?Sized,
//     <B as ToOwned>::Owned: Default,
// 1.7.0 · source
// impl<K, V, S> Default for HashMap<K, V, S>where
//     S: Default,
// source
// impl<T> Default for &[T]
// 1.5.0 · source
// impl<T> Default for &mut [T]
// source
// impl<T> Default for Option<T>
// 1.4.0 · source
// impl<T> Default for [T; 0]
// 1.4.0 · source
// impl<T> Default for [T; 1]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 2]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 3]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 4]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 5]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 6]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 7]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 8]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 9]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 10]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 11]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 12]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 13]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 14]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 15]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 16]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 17]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 18]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 19]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 20]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 21]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 22]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 23]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 24]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 25]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 26]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 27]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 28]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 29]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 30]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 31]where
//     T: Default,
// 1.4.0 · source
// impl<T> Default for [T; 32]where
//     T: Default,
// impl<T> Default for Cell<T>where
//     T: Default,
// source
// impl<T> Default for LazyCell<T, fn() -> T>where
//     T: Default,
// 1.70.0 · source
// impl<T> Default for OnceCell<T>
// source
// impl<T> Default for RefCell<T>where
//     T: Default,
// source
// impl<T> Default for SyncUnsafeCell<T>where
//     T: Default,
// 1.10.0 · source
// impl<T> Default for UnsafeCell<T>where
//     T: Default,
// 1.19.0 · source
// impl<T> Default for Reverse<T>where
//     T: Default,
// 1.70.0 · source
// impl<T> Default for std::collections::binary_heap::IntoIter<T, Global>
// 1.70.0 · source
// impl<T> Default for std::collections::btree_set::Iter<'_, T>
// 1.70.0 · source
// impl<T> Default for std::collections::btree_set::Range<'_, T>
// 1.70.0 · source
// impl<T> Default for std::collections::linked_list::IntoIter<T, Global>
// 1.70.0 · source
// impl<T> Default for std::collections::linked_list::Iter<'_, T>
// 1.70.0 · source
// impl<T> Default for std::collections::linked_list::IterMut<'_, T>
// source
// impl<T> Default for BTreeSet<T, Global>
// source
// impl<T> Default for BinaryHeap<T, Global>where
//     T: Ord,
// source
// impl<T> Default for LinkedList<T, Global>
// source
// impl<T> Default for VecDeque<T, Global>
// 1.2.0 · source
// impl<T> Default for std::iter::Empty<T>
// source
// impl<T> Default for PhantomData<T>where
//     T: ?Sized,
// 1.20.0 · source
// impl<T> Default for ManuallyDrop<T>where
//     T: Default + ?Sized,
// source
// impl<T> Default for Saturating<T>where
//     T: Default,
// source
// impl<T> Default for Wrapping<T>where
//     T: Default,
// 1.62.0 · source
// impl<T> Default for AssertUnwindSafe<T>where
//     T: Default,
// source
// impl<T> Default for Rc<T, Global>where
//     T: Default,
// 1.10.0 · source
// impl<T> Default for std::rc::Weak<T, Global>
// 1.70.0 · source
// impl<T> Default for std::slice::Iter<'_, T>
// 1.70.0 · source
// impl<T> Default for std::slice::IterMut<'_, T>
// source
// impl<T> Default for AtomicPtr<T>
// source
// impl<T> Default for Arc<T, Global>where
//     T: Default,
// source
// impl<T> Default for Exclusive<T>where
//     T: Default + ?Sized,
// impl<T> Default for OnceLock<T>
// impl<T> Default for std::sync::Weak<T, Global>
// impl<T> Default for Vec<T, Global>
// impl<T, A> Default for std::collections::btree_set::IntoIter<T, A>where
//     A: Allocator + Default + Clone,
// 1.70.0 · source
// impl<T, A> Default for std::vec::IntoIter<T, A>where
//     A: Allocator + Default,
// source
// impl<T, S> Default for HashSet<T, S>where
//     S: Default,
// source
// impl<T, const LANES: usize> Default for Mask<T, LANES>where
//     T: MaskElement,
//     LaneCount<LANES>: SupportedLaneCount,
// source
// impl<T, const N: usize> Default for Simd<T, N>where
//     LaneCount<N>: SupportedLaneCount,
//     T: SimdElement + Default,
//
// impl<T: Default> Default for Cursor<T>
// impl<T: Default> Default for LazyLock<T>
// impl<T: Default> Default for RwLock<T>
// impl<T: ?Sized + Default> Default for Mutex<T>
//
// impl Default for TokenStream
// impl Default for TestTimeOptions
// impl Default for OutputFormat
// impl Default for ColorConfig

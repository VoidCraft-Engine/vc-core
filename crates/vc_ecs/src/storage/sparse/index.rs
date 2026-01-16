use core::hash::Hash;
// -----------------------------------------------------------------------------
// SparseIndex

pub trait SparseIndex: Copy + PartialEq + Eq + Hash {
    /// Gets the sparse set index corresponding to this instance.
    fn sparse_index(&self) -> usize;
}

// -----------------------------------------------------------------------------
// Implementation

impl SparseIndex for crate::component::ComponentId {
    #[inline(always)]
    fn sparse_index(&self) -> usize {
        self.index()
    }
}

// -----------------------------------------------------------------------------
// Primitive

// macro_rules! impl_sparse_set_index {
//     ($($ty:ty),+) => {
//         $(impl SparseIndex for $ty {
//             #[inline(always)]
//             fn sparse_set_index(&self) -> usize {
//                 *self as usize
//             }
//         })*
//     };
// }

// impl_sparse_set_index!(u8, u16, u32, u64, usize);

// -----------------------------------------------------------------------------
// WorldId

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct WorldId(u64);

impl WorldId {
    #[inline(always)]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

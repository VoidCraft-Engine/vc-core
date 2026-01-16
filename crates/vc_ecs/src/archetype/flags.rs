bitflags::bitflags! {
    /// Flags used to keep track of metadata about the component in
    /// this `Archetype`
    ///
    /// Used primarily to early-out when there are no `ComponentHook`
    /// registered for any contained components.
    #[derive(Clone, Copy)]
    pub struct ArchetypeFlags: u32 {
        const ON_ADD_HOOK    = (1 << 0);
        const ON_INSERT_HOOK = (1 << 1);
        const ON_REPLACE_HOOK = (1 << 2);
        const ON_REMOVE_HOOK = (1 << 3);
        const ON_DESPAWN_HOOK = (1 << 4);
        const ON_ADD_OBSERVER = (1 << 5);
        const ON_INSERT_OBSERVER = (1 << 6);
        const ON_REPLACE_OBSERVER = (1 << 7);
        const ON_REMOVE_OBSERVER = (1 << 8);
        const ON_DESPAWN_OBSERVER = (1 << 9);
    }
}

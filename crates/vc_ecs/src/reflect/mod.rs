use core::ops::{Deref, DerefMut};

use vc_reflect::registry::TypeRegistryArc;

mod component;

#[derive(Clone, Default)]
pub struct AppTypeRegistry(TypeRegistryArc);

impl Deref for AppTypeRegistry {
    type Target = TypeRegistryArc;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppTypeRegistry {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

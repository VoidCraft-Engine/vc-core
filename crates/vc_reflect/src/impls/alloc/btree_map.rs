use crate::{
    FromReflect, Reflect,
    derive::impl_type_path,
    impls::GenericTypeInfoCell,
    info::{GenericInfo, Generics, MapInfo, TypeInfo, TypeParamInfo, Typed},
    ops::{ApplyError, Map, ReflectCloneError},
    registry::{
        FromType, GetTypeMeta, TypeMeta, TypeTraitDefault, TypeTraitFromPtr, TypeTraitFromReflect,
    },
};
use alloc::{boxed::Box, vec::Vec};

impl_type_path!(::alloc::collections::BTreeMap<K, V>);

impl<K, V> Typed for ::alloc::collections::BTreeMap<K, V>
where
    K: FromReflect + Typed + Eq + Ord,
    V: FromReflect + Typed,
{
    fn type_info() -> &'static TypeInfo {
        static CELL: GenericTypeInfoCell = GenericTypeInfoCell::new();
        CELL.get_or_insert::<Self>(|| {
            TypeInfo::Map(
                MapInfo::new::<Self, K, V>().with_generics(Generics::from_iter([
                    GenericInfo::Type(TypeParamInfo::new::<K>("K")),
                    GenericInfo::Type(TypeParamInfo::new::<V>("V")),
                ])),
            )
        })
    }
}

impl<K, V> Reflect for ::alloc::collections::BTreeMap<K, V>
where
    K: FromReflect + Typed + Eq + Ord,
    V: FromReflect + Typed,
{
    crate::reflection::impl_reflect_cast_fn!(Map);

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::map_try_apply(self, value)
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Map>::to_dynamic_map(self))
    }

    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        let mut map = Self::new();
        for (key, value) in Self::iter(self) {
            let key = key
                .reflect_clone()?
                .take::<K>()
                .expect("`Reflect::reflect_clone` should return the same type");
            let value = value
                .reflect_clone()?
                .take::<V>()
                .expect("`Reflect::reflect_clone` should return the same type");
            map.insert(key, value);
        }

        Ok(Box::new(map))
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::map_hash(self)
    }

    #[inline]
    fn reflect_partial_eq(&self, value: &dyn Reflect) -> Option<bool> {
        crate::impls::map_partial_eq(self, value)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::impls::map_debug(self, f)
    }
}

impl<K, V> Map for ::alloc::collections::BTreeMap<K, V>
where
    K: FromReflect + Typed + Eq + Ord,
    V: FromReflect + Typed,
{
    fn get(&self, key: &dyn Reflect) -> Option<&dyn Reflect> {
        key.downcast_ref::<K>()
            .and_then(|key| Self::get(self, key))
            .map(|value| value as &dyn Reflect)
    }

    fn get_mut(&mut self, key: &dyn Reflect) -> Option<&mut dyn Reflect> {
        key.downcast_ref::<K>()
            .and_then(move |key| Self::get_mut(self, key))
            .map(|value| value as &mut dyn Reflect)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }

    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
    }

    #[inline]
    fn iter(&self) -> Box<dyn Iterator<Item = (&dyn Reflect, &dyn Reflect)> + '_> {
        Box::new(Self::iter(self).map(|(k, v)| (k as &dyn Reflect, v as &dyn Reflect)))
    }

    fn drain(&mut self) -> Vec<(Box<dyn Reflect>, Box<dyn Reflect>)> {
        let mut result = Vec::with_capacity(self.len());
        while let Some((k, v)) = self.pop_first() {
            result.push((
                Box::new(k) as Box<dyn Reflect>,
                Box::new(v) as Box<dyn Reflect>,
            ));
        }
        result
    }

    fn retain(&mut self, f: &mut dyn FnMut(&dyn Reflect, &mut dyn Reflect) -> bool) {
        Self::retain(self, move |k, v| f(k, v));
    }

    fn insert_boxed(
        &mut self,
        key: Box<dyn Reflect>,
        value: Box<dyn Reflect>,
    ) -> Option<Box<dyn Reflect>> {
        let key = K::take_from_reflect(key).unwrap_or_else(|key| {
            panic!(
                "Attempted to insert invalid key of type {}.",
                key.reflect_type_path()
            )
        });
        let value = V::take_from_reflect(value).unwrap_or_else(|value| {
            panic!(
                "Attempted to insert invalid value of type {}.",
                value.reflect_type_path()
            )
        });
        self.insert(key, value)
            .map(|old_val| Box::new(old_val) as Box<dyn Reflect>)
    }

    fn remove(&mut self, key: &dyn Reflect) -> Option<Box<dyn Reflect>> {
        let mut from_reflect = None;
        key.downcast_ref::<K>()
            .or_else(|| {
                from_reflect = K::from_reflect(key);
                from_reflect.as_ref()
            })
            .and_then(|key| self.remove(key))
            .map(|value| Box::new(value) as Box<dyn Reflect>)
    }
}

impl<K, V> FromReflect for ::alloc::collections::BTreeMap<K, V>
where
    K: FromReflect + Typed + Eq + Ord,
    V: FromReflect + Typed,
{
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        let ref_map = reflect.reflect_ref().as_map().ok()?;

        let mut new_map = Self::new();

        for (key, value) in ref_map.iter() {
            let new_key = K::from_reflect(key)?;
            let new_value = V::from_reflect(value)?;
            new_map.insert(new_key, new_value);
        }

        Some(new_map)
    }
}

impl<K, V> GetTypeMeta for ::alloc::collections::BTreeMap<K, V>
where
    K: FromReflect + Typed + GetTypeMeta + Eq + Ord,
    V: FromReflect + Typed + GetTypeMeta,
{
    fn get_type_meta() -> TypeMeta {
        let mut type_meta = TypeMeta::with_capacity::<Self>(3);
        type_meta.insert_trait::<TypeTraitFromPtr>(FromType::<Self>::from_type());
        type_meta.insert_trait::<TypeTraitFromReflect>(FromType::<Self>::from_type());
        type_meta.insert_trait::<TypeTraitDefault>(FromType::<Self>::from_type());
        type_meta
    }

    fn register_dependencies(registry: &mut crate::registry::TypeRegistry) {
        registry.register::<K>();
        registry.register::<V>();
    }
}

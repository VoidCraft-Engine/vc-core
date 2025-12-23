use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    format,
    string::String,
};
use core::fmt;

use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{EnumInfo, OpaqueInfo, TypeInfo, TypePath, Typed, VariantKind},
    ops::{
        ApplyError, DynamicStruct, DynamicTuple, DynamicVariant, ReflectCloneError, Struct, Tuple,
        VariantFieldIter,
    },
    reflection::impl_reflect_cast_fn,
};

/// A dynamic representation of an enum, allows for enums to be configured at runtime.
///
/// Dynamic types are special in that their TypeInfo is [`OpaqueInfo`],
/// but other APIs are consistent with the type they represent, such as [`reflect_kind`], [`reflect_ref`]
///
/// # Example
///
/// ```
/// # use vc_reflect::{Reflect, ops::{DynamicEnum, DynamicVariant}};
///
/// // The original enum value
/// let mut value: Option<usize> = Some(123);
///
/// // Create a DynamicEnum to represent the new value
/// let mut dyn_enum = DynamicEnum::new(
///   "None",
///   DynamicVariant::Unit
/// );
///
/// // Apply the DynamicEnum as a patch to the original value
/// value.apply(dyn_enum.as_reflect());
///
/// // Tada!
/// assert_eq!(None, value);
/// ```
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicEnum {
    enum_info: Option<&'static TypeInfo>,
    variant_index: usize,
    variant_name: Cow<'static, str>,
    variant: DynamicVariant,
}

impl TypePath for DynamicEnum {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicEnum"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicEnum"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicEnum"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicEnum {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicEnum {
    /// Create a new [`TypeInfo`] to represent an enum at runtime.
    #[inline]
    pub fn new<I: Into<Cow<'static, str>>, V: Into<DynamicVariant>>(
        variant_name: I,
        variant: V,
    ) -> Self {
        Self {
            enum_info: None,
            variant_index: 0,
            variant_name: variant_name.into(),
            variant: variant.into(),
        }
    }

    /// Create a new [`DynamicEnum`] with a variant index to represent an enum at runtime.
    #[inline]
    pub fn new_with_index<I: Into<Cow<'static, str>>, V: Into<DynamicVariant>>(
        variant_index: usize,
        variant_name: I,
        variant: V,
    ) -> Self {
        Self {
            enum_info: None,
            variant_index,
            variant_name: variant_name.into(),
            variant: variant.into(),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicEnum`.
    ///
    /// # Panic
    ///
    /// If the input is not enum info or None.
    #[inline]
    pub fn set_type_info(&mut self, enum_info: Option<&'static TypeInfo>) {
        match enum_info {
            Some(TypeInfo::Enum(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicEnum::set_type_info`, but the input is not enum information or None."
                )
            }
        }

        self.enum_info = enum_info;
    }

    /// Set the current enum variant represented by this struct.
    #[inline]
    pub fn set_variant<I: Into<Cow<'static, str>>, V: Into<DynamicVariant>>(
        &mut self,
        name: I,
        variant: V,
    ) {
        self.variant_name = name.into();
        self.variant = variant.into();
    }

    /// Set the current enum variant represented by this struct along with its variant index.
    #[inline]
    pub fn set_variant_with_index<I: Into<Cow<'static, str>>, V: Into<DynamicVariant>>(
        &mut self,
        variant_index: usize,
        variant_name: I,
        variant: V,
    ) {
        self.variant_index = variant_index;
        self.variant_name = variant_name.into();
        self.variant = variant.into();
    }

    /// Get a reference to the [`DynamicVariant`] contained in `self`.
    #[inline]
    pub fn variant(&self) -> &DynamicVariant {
        &self.variant
    }

    /// Get a mutable reference to the [`DynamicVariant`] contained in `self`.
    ///
    /// Using the mut reference to switch to a different variant will ___not___ update the
    /// internal tracking of the variant name and index.
    ///
    /// If you want to switch variants, prefer one of the setters:
    /// [`DynamicEnum::set_variant`] or [`DynamicEnum::set_variant_with_index`].
    #[inline]
    pub fn variant_mut(&mut self) -> &mut DynamicVariant {
        &mut self.variant
    }

    /// Create a [`DynamicEnum`] from an existing one.
    ///
    /// This is functionally the same as [`DynamicEnum::from_ref`] except it takes an owned value.
    #[inline]
    pub fn from<TEnum: Enum>(value: TEnum) -> Self {
        // copy value instead of referencing
        Self::from_ref(&value)
    }

    /// Create a [`DynamicEnum`] from an existing one.
    ///
    /// This is functionally the same as [`DynamicEnum::from`] except it takes a reference.
    pub fn from_ref<TEnum: Enum + ?Sized>(value: &TEnum) -> Self {
        let mut dyn_enum = match value.variant_kind() {
            VariantKind::Unit => DynamicEnum::new_with_index(
                value.variant_index(),
                value.variant_name().to_owned(),
                DynamicVariant::Unit,
            ),
            VariantKind::Tuple => {
                let mut data = DynamicTuple::new();
                for field in value.iter_fields() {
                    data.insert_boxed(field.value().to_dynamic());
                }
                DynamicEnum::new_with_index(
                    value.variant_index(),
                    value.variant_name().to_owned(),
                    DynamicVariant::Tuple(data),
                )
            }
            VariantKind::Struct => {
                let mut data = DynamicStruct::new();
                for field in value.iter_fields() {
                    let name = field.name().unwrap();
                    data.insert_boxed(name.to_owned(), field.value().to_dynamic());
                }
                DynamicEnum::new_with_index(
                    value.variant_index(),
                    value.variant_name().to_owned(),
                    DynamicVariant::Struct(data),
                )
            }
        };

        dyn_enum.set_type_info(value.represented_type_info());
        dyn_enum
    }
}

impl Reflect for DynamicEnum {
    impl_reflect_cast_fn!(Enum);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.enum_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Enum>::to_dynamic_enum(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as Enum>::to_dynamic_enum(self)))
    }

    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        if let Some(y) = crate::impls::enum_try_apply(self, value)? {
            let dyn_variant = match y.variant_kind() {
                VariantKind::Unit => DynamicVariant::Unit,
                VariantKind::Tuple => {
                    let mut dyn_tuple = DynamicTuple::new();
                    for y_field in y.iter_fields() {
                        dyn_tuple.insert_boxed(y_field.value().to_dynamic());
                    }
                    DynamicVariant::Tuple(dyn_tuple)
                }
                VariantKind::Struct => {
                    let mut dyn_struct = DynamicStruct::new();
                    for y_field in y.iter_fields() {
                        dyn_struct.insert_boxed(
                            y_field.name().unwrap().to_owned(),
                            y_field.value().to_dynamic(),
                        );
                    }
                    DynamicVariant::Struct(dyn_struct)
                }
            };
            self.set_variant(y.variant_name().to_owned(), dyn_variant);
        }
        Ok(())
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::enum_partial_eq(self, other)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::enum_hash(self)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicEnum(")?;
        crate::impls::enum_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicEnum {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

/// A trait used to power [enum-like] operations via [reflection].
///
/// This allows enums to be processed and modified dynamically at runtime without
/// necessarily knowing the actual type.
/// Enums are much more complex than their struct counterparts.
/// As a result, users will need to be mindful of conventions, considerations,
/// and complications when working with this trait.
///
/// # Variants
///
/// An enum is a set of choices called _variants_.
/// An instance of an enum can only exist as one of these choices at any given time.
/// Consider Rust's [`Option<T>`]. It's an enum with two variants: [`None`] and [`Some`].
/// If you're `None`, you can't be `Some` and vice versa.
///
/// > ⚠️ __This is very important:__
/// > The [`Enum`] trait represents an enum _as one of its variants_.
/// > It does not represent the entire enum since that's not true to how enums work.
///
/// Variants come in a few [flavors](VariantType):
///
/// | Variant Type | Syntax                         |
/// | ------------ | ------------------------------ |
/// | Unit         | `MyEnum::Foo`                  |
/// | Tuple        | `MyEnum::Foo( i32, i32 )`      |
/// | Struct       | `MyEnum::Foo{ value: String }` |
///
/// As you can see, a unit variant contains no fields, while tuple and struct variants
/// can contain one or more fields.
/// The fields in a tuple variant is defined by their _order_ within the variant.
/// Index `0` represents the first field in the variant and so on.
/// Fields in struct variants (excluding tuple structs), on the other hand, are
/// represented by a _name_.
///
/// # Implementation
///
/// > 💡 This trait can be automatically implemented using [`#[derive(Reflect)]`](crate::derive::Reflect)
/// > on an enum definition.
///
/// Despite the fact that enums can represent multiple states, traits only exist in one state
/// and must be applied to the entire enum rather than a particular variant.
/// Because of this limitation, the [`Enum`] trait must not only _represent_ any of the
/// three variant types, but also define the _methods_ for all three as well.
///
/// What does this mean? It means that even though a unit variant contains no fields, a
/// representation of that variant using the [`Enum`] trait will still contain methods for
/// accessing fields!
/// Again, this is to account for _all three_ variant types.
///
/// We recommend using the built-in [`#[derive(Reflect)]`](crate::derive::Reflect) macro to automatically handle all the
/// implementation details for you.
/// However, if you _must_ implement this trait manually, there are a few things to keep in mind...
///
/// ## Field Order
///
/// While tuple variants identify their fields by the order in which they are defined, struct
/// variants identify fields by their name.
/// However, both should allow access to fields by their defined order.
///
/// The reason all fields, regardless of variant type, need to be accessible by their order is
/// due to field iteration.
/// We need a way to iterate through each field in a variant, and the easiest way of achieving
/// that is through the use of field order.
///
/// The derive macro adds proper struct variant handling for [`Enum::index_of`], [`Enum::name_at`]
/// and [`Enum::field_at[_mut]`](Enum::field_at) methods.
/// The first two methods are __required__ for all struct variant types.
/// By convention, implementors should also handle the last method as well, but this is not
/// a strict requirement.
///
/// ## Field Names
///
/// Implementors may choose to handle [`Enum::index_of`], [`Enum::name_at`], and
/// [`Enum::field[_mut]`](Enum::field) for tuple variants by considering stringified `usize`s to be
/// valid names (such as `"3"`).
/// This isn't wrong to do, but the convention set by the derive macro is that it isn't supported.
/// It's preferred that these strings be converted to their proper `usize` representations and
/// the [`Enum::field_at[_mut]`](Enum::field_at) methods be used instead.
///
/// [enum-like]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
/// [reflection]: crate
/// [`None`]: Option<T>::None
/// [`Some`]: Option<T>::Some
pub trait Enum: Reflect {
    /// Returns a reference to the value of the field (in the current variant) with the given name.
    ///
    /// For non-[`VariantType::Struct`] variants, this should return `None`.
    fn field(&self, name: &str) -> Option<&dyn Reflect>;

    /// Returns a reference to the value of the field (in the current variant) at the given index.
    fn field_at(&self, index: usize) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value of the field (in the current variant) with the given name.
    ///
    /// For non-[`VariantType::Struct`] variants, this should return `None`.
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect>;

    /// Returns a mutable reference to the value of the field (in the current variant) at the given index.
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    /// Returns the index of the field (in the current variant) with the given name.
    ///
    /// For non-[`VariantType::Struct`] variants, this should return `None`.
    fn index_of(&self, name: &str) -> Option<usize>;

    /// Returns the name of the field (in the current variant) with the given index.
    ///
    /// For non-[`VariantType::Struct`] variants, this should return `None`.
    fn name_at(&self, index: usize) -> Option<&str>;

    /// Returns an iterator over the values of the current variant's fields.
    fn iter_fields(&self) -> VariantFieldIter<'_>;

    /// Returns the number of fields in the current variant.
    fn field_len(&self) -> usize;

    /// The name of the current variant.
    fn variant_name(&self) -> &str;

    /// Returns the full path to the current variant.
    fn variant_path(&self) -> String {
        format!("{}::{}", self.reflect_type_path(), self.variant_name())
    }

    /// The index of the current variant.
    fn variant_index(&self) -> usize;

    /// The type of the current variant.
    fn variant_kind(&self) -> VariantKind;

    /// Creates a new [`DynamicEnum`] from this enum.
    #[inline]
    fn to_dynamic_enum(&self) -> DynamicEnum {
        DynamicEnum::from_ref(self)
    }

    /// Returns true if the current variant's type matches the given one.
    #[inline]
    fn is_variant(&self, variant_kind: VariantKind) -> bool {
        self.variant_kind() == variant_kind
    }

    /// Get actual [`EnumInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `EnumInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_enum_info(&self) -> Option<&'static EnumInfo> {
        self.reflect_type_info().as_enum().ok()
    }

    /// Get the [`EnumInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_enum_info(&self) -> Option<&'static EnumInfo> {
        self.represented_type_info()?.as_enum().ok()
    }
}

impl Enum for DynamicEnum {
    fn field(&self, name: &str) -> Option<&dyn Reflect> {
        if let DynamicVariant::Struct(data) = &self.variant {
            data.field(name)
        } else {
            None
        }
    }

    fn field_at(&self, index: usize) -> Option<&dyn Reflect> {
        match &self.variant {
            DynamicVariant::Tuple(data) => data.field(index),
            DynamicVariant::Struct(data) => data.field_at(index),
            DynamicVariant::Unit => None,
        }
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect> {
        if let DynamicVariant::Struct(data) = &mut self.variant {
            data.field_mut(name)
        } else {
            None
        }
    }

    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        match &mut self.variant {
            DynamicVariant::Tuple(data) => data.field_mut(index),
            DynamicVariant::Struct(data) => data.field_at_mut(index),
            DynamicVariant::Unit => None,
        }
    }

    fn index_of(&self, name: &str) -> Option<usize> {
        if let DynamicVariant::Struct(data) = &self.variant {
            data.index_of(name)
        } else {
            None
        }
    }

    fn name_at(&self, index: usize) -> Option<&str> {
        if let DynamicVariant::Struct(data) = &self.variant {
            data.name_at(index)
        } else {
            None
        }
    }

    #[inline]
    fn iter_fields(&self) -> VariantFieldIter<'_> {
        VariantFieldIter::new(self)
    }

    fn field_len(&self) -> usize {
        match &self.variant {
            DynamicVariant::Unit => 0,
            DynamicVariant::Tuple(data) => data.field_len(),
            DynamicVariant::Struct(data) => data.field_len(),
        }
    }

    #[inline]
    fn variant_name(&self) -> &str {
        &self.variant_name
    }

    #[inline]
    fn variant_index(&self) -> usize {
        self.variant_index
    }

    #[inline]
    fn variant_kind(&self) -> VariantKind {
        match &self.variant {
            DynamicVariant::Unit => VariantKind::Unit,
            DynamicVariant::Tuple(..) => VariantKind::Tuple,
            DynamicVariant::Struct(..) => VariantKind::Struct,
        }
    }

    #[inline]
    fn reflect_enum_info(&self) -> Option<&'static EnumInfo> {
        None
    }

    #[inline]
    fn represented_enum_info(&self) -> Option<&'static EnumInfo> {
        self.enum_info?.as_enum().ok()
    }
}

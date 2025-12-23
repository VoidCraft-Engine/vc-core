/// Implements `docs` and `with_docs` helpers.
macro_rules! impl_docs_fn {
    ($field:ident) => {
        /// Returns the docs, if present.
        ///
        /// If `reflect_docs` feature is not enabled, this function always return `None`.
        /// So you can use this without worrying about compilation options.
        #[inline(always)]
        pub const fn docs(&self) -> Option<&'static str> {
            #[cfg(not(feature = "reflect_docs"))]
            return None;
            #[cfg(feature = "reflect_docs")]
            return self.$field;
        }

        /// Replaces docs (overwrite, do not merge).
        ///
        /// Used by the proc-macro crate.
        #[cfg(feature = "reflect_docs")]
        #[inline]
        pub fn with_docs(self, $field: Option<&'static str>) -> Self {
            Self { $field, ..self }
        }
    };
}

pub(crate) use impl_docs_fn;

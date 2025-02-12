//! Module with the common definitions for  the
//! [fields](https://github.com/typesense/typesense/blob/v0.19.0/include/field.)
//! available in Typesense.

mod field_type;
pub use field_type::*;
pub use typesense_codegen::models::{Field, FieldEmbed};

/// Builder for the `Field` struct.
#[derive(Debug, Default)]
pub struct FieldBuilder {
    inner: Field,
}

impl FieldBuilder {
    /// Create a Builder
    #[inline]
    pub fn new(name: impl Into<String>, typesense_type: FieldType) -> Self {
        Self {
            inner: Field::new(name.into(), typesense_type),
        }
    }

    /// Set if field is optional.
    #[inline]
    pub fn optional(mut self, optional: Option<bool>) -> Self {
        self.inner.optional = optional;
        self
    }

    /// Set if field is facet.
    #[inline]
    pub fn facet(mut self, facet: Option<bool>) -> Self {
        self.inner.facet = facet;
        self
    }

    /// Set if field is index.
    #[inline]
    pub fn index(mut self, index: Option<bool>) -> Self {
        self.inner.index = index;
        self
    }

    /// Set field locale.
    #[inline]
    pub fn locale(mut self, locale: Option<String>) -> Self {
        self.inner.locale = locale;
        self
    }

    /// Set sort attribute for field
    #[inline]
    pub fn sort(mut self, sort: Option<bool>) -> Self {
        self.inner.sort = sort;
        self
    }

    /// Set infix attribute for field
    #[inline]
    pub fn infix(mut self, infix: Option<bool>) -> Self {
        self.inner.infix = infix;
        self
    }

    /// Set num_dim attribute for field
    #[inline]
    pub fn num_dim(mut self, num_dim: Option<i32>) -> Self {
        self.inner.num_dim = num_dim;
        self
    }

    /// Set drop attribute for field
    #[inline]
    pub fn drop(mut self, drop: Option<bool>) -> Self {
        self.inner.drop = drop;
        self
    }

    /// Create a `Field` with the current values of the builder,
    /// It can fail if the name or the typesense_type are not defined.
    #[inline]
    pub fn build(self) -> Field {
        self.inner
    }
}

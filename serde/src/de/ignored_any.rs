use lib::*;

use de::{Deserialize, Deserializer, Visitor, SeqVisitor, MapVisitor, Error};

/// An efficient way of discarding data from a deserializer.
///
/// Think of this like `serde_json::Value` in that it can be deserialized from
/// any type, except that it does not store any information about the data that
/// gets deserialized.
///
/// ```rust
/// use std::fmt;
/// use std::marker::PhantomData;
///
/// use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, Visitor, SeqVisitor, IgnoredAny};
///
/// /// A seed that can be used to deserialize only the `n`th element of a sequence
/// /// while efficiently discarding elements of any type before or after index `n`.
/// ///
/// /// For example to deserialize only the element at index 3:
/// ///
/// /// ```rust
/// /// NthElement::new(3).deserialize(deserializer)
/// /// ```
/// pub struct NthElement<T> {
///     n: usize,
///     marker: PhantomData<T>,
/// }
///
/// impl<T> NthElement<T> {
///     pub fn new(n: usize) -> Self {
///         NthElement {
///             n: n,
///             marker: PhantomData,
///         }
///     }
/// }
///
/// impl<'de, T> Visitor<'de> for NthElement<T>
///     where T: Deserialize<'de>
/// {
///     type Value = T;
///
///     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
///         write!(formatter, "a sequence in which we care about element {}", self.n)
///     }
///
///     fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
///         where V: SeqVisitor<'de>
///     {
///         // Skip over the first `n` elements.
///         for i in 0..self.n {
///             // It is an error if the sequence ends before we get to element `n`.
///             if seq.visit::<IgnoredAny>()?.is_none() {
///                 return Err(de::Error::invalid_length(i, &self));
///             }
///         }
///
///         // Deserialize the one we care about.
///         let nth = match seq.visit()? {
///             Some(nth) => nth,
///             None => {
///                 return Err(de::Error::invalid_length(self.n, &self));
///             }
///         };
///
///         // Skip over any remaining elements in the sequence after `n`.
///         while let Some(IgnoredAny) = seq.visit()? {
///             // ignore
///         }
///
///         Ok(nth)
///     }
/// }
///
/// impl<'de, T> DeserializeSeed<'de> for NthElement<T>
///     where T: Deserialize<'de>
/// {
///     type Value = T;
///
///     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
///         where D: Deserializer<'de>
///     {
///         deserializer.deserialize_seq(self)
///     }
/// }
///
/// # fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
/// #     where D: Deserializer<'de>
/// # {
/// // Deserialize only the sequence element at index 3 from this deserializer.
/// // The element at index 3 is required to be a string. Elements before and
/// // after index 3 are allowed to be of any type.
/// let s: String = NthElement::new(3).deserialize(deserializer)?;
/// #     Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct IgnoredAny;

impl<'de> Visitor<'de> for IgnoredAny {
    type Value = IgnoredAny;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("anything at all")
    }

    #[inline]
    fn visit_bool<E>(self, x: bool) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_i64<E>(self, x: i64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_u64<E>(self, x: u64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_f64<E>(self, x: f64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = s;
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        IgnoredAny::deserialize(deserializer)
    }

    #[inline]
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        IgnoredAny::deserialize(deserializer)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqVisitor<'de>,
    {
        while let Some(IgnoredAny) = try!(visitor.visit()) {
            // Gobble
        }
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapVisitor<'de>,
    {
        while let Some((IgnoredAny, IgnoredAny)) = try!(visitor.visit()) {
            // Gobble
        }
        Ok(IgnoredAny)
    }

    #[inline]
    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = bytes;
        Ok(IgnoredAny)
    }
}

impl<'de> Deserialize<'de> for IgnoredAny {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<IgnoredAny, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_ignored_any(IgnoredAny)
    }
}

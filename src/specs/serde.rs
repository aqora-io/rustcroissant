use core::fmt;
use std::marker::PhantomData;

use serde::{
    Deserialize, Deserializer,
    de::{self, IntoDeserializer, SeqAccess, Visitor},
};

pub fn one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct OneOrManyVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for OneOrManyVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a value or a list of values")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut out = Vec::new();
            while let Some(item) = seq.next_element::<T>()? {
                out.push(item);
            }
            Ok(out)
        }

        fn visit_map<A>(self, map: A) -> Result<Vec<T>, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            // Treat a single object as a single T
            let single = T::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(vec![single])
        }

        fn visit_str<E>(self, v: &str) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_string<E>(self, v: String) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_bool<E>(self, v: bool) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_i64<E>(self, v: i64) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_u64<E>(self, v: u64) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_f64<E>(self, v: f64) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            let single = T::deserialize(v.into_deserializer())?;
            Ok(vec![single])
        }

        fn visit_unit<E>(self) -> Result<Vec<T>, E>
        where
            E: de::Error,
        {
            Err(E::custom("null is not valid for one-or-many"))
        }
    }

    deserializer.deserialize_any(OneOrManyVisitor::<T>(PhantomData))
}

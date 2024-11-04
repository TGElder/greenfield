use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub trait ContainsKeyValue {
    type Key;
    type Value;

    fn contains_key_value<K: Borrow<Self::Key>, V: Borrow<Self::Value>>(
        &self,
        key: K,
        value: V,
    ) -> bool;
}

impl<T, U> ContainsKeyValue for HashMap<T, U>
where
    T: Eq + Hash,
    U: PartialEq,
{
    type Key = T;
    type Value = U;

    fn contains_key_value<K: Borrow<Self::Key>, V: Borrow<Self::Value>>(
        &self,
        key: K,
        value: V,
    ) -> bool {
        self.get(key.borrow()) == Some(value.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_key_value() {
        // given
        let map: HashMap<u32, u32> = HashMap::from_iter([(123, 456)]);

        // then
        assert!(map.contains_key_value(123, 456));
        assert!(map.contains_key_value(&123, &456));
    }

    #[test]
    fn contains_key_with_different_value() {
        // given
        let map: HashMap<u32, u32> = HashMap::from_iter([(123, 123)]);

        // when
        assert!(!map.contains_key_value(123, 456));
        assert!(!map.contains_key_value(&123, &456));
    }

    #[test]
    fn does_not_contain_key() {
        // given
        let map: HashMap<u32, u32> = HashMap::from_iter([(456, 123)]);

        // then
        assert!(!map.contains_key_value(123, 456));
    }
}

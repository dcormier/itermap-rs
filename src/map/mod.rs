mod keys;
mod values;

pub use keys::MapKeys;
pub use values::MapValues;

#[cfg(test)]
mod test {
    use crate::IterMap;

    #[test]
    fn double_ended() {
        let original = vec![("a", "A"), ("b", "B"), ("c", "C")];

        // If this doesn't compile, `MapKeys` doesn't implement `DoubleEndedIterator`
        let mapped: Vec<_> = original
            .clone()
            .into_iter()
            .map_keys(|k| k.to_uppercase())
            .rev()
            .collect();

        assert_eq!(
            vec![("C", "C"), ("B", "B"), ("A", "A")]
                .into_iter()
                .map_keys(String::from)
                .collect::<Vec<_>>(),
            mapped
        );

        // If this doesn't compile, `MapValues` doesn't implement `DoubleEndedIterator`
        let mapped: Vec<_> = original
            .into_iter()
            .map_values(|v| v.to_lowercase())
            .rev()
            .collect();

        assert_eq!(
            vec![("c", "c"), ("b", "b"), ("a", "a")]
                .into_iter()
                .map_values(String::from)
                .collect::<Vec<_>>(),
            mapped
        );
    }

    #[test]
    fn exact_size() {
        let original = vec![("a", "A"), ("b", "B"), ("c", "C")];

        // If this doesn't compile, `MapKeys` doesn't implement `ExactSizeIterator`
        let len = original.clone().into_iter().map_keys(|k| k).len();
        assert_eq!(3, len);

        // If this doesn't compile, `MapValues` doesn't implement `ExactSizeIterator`
        let len = original.into_iter().map_values(|v| v).len();
        assert_eq!(3, len);
    }
}

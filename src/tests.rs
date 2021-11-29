use crate::IterMap;

/// For testing things that intentionally will not work.
///
/// ----
///
/// Iterator<Item = T>
///
/// map_keys
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<&str> = Vec::new();
/// vec.push("a");
/// vec.push("b");
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_keys(String::from)
///     .collect();
/// ```
///
/// map_values
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<&str> = Vec::new();
/// vec.push("a");
/// vec.push("b");
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_values(String::from)
///     .collect();
/// ```
///
/// ----
///
/// Iterator<Item = (T)>
///
/// map_keys
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<(&str)> = Vec::new();
/// vec.push(("a"));
/// vec.push(("b"));
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_keys(String::from)
///     .collect();
/// ```
///
/// map_values
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<(&str)> = Vec::new();
/// vec.push(("a"));
/// vec.push(("b"));
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_values(String::from)
///     .collect();
/// ```
/// ----
///
/// Iterator<Item = (T, U, V)>
///
/// map_keys
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<(&str, &str, &str)> = Vec::new();
/// vec.push(("a", "A", "1"));
/// vec.push(("b", "B", "2"));
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_keys(String::from)
///     .collect();
/// ```
///
/// map_values
/// ``` compile_fail
/// use std::collections::HashMap;
///
/// use itermap::IterMap;
///
/// let mut vec: Vec<(&str, &str, &str)> = Vec::new();
/// vec.push(("a", "A", "1"));
/// vec.push(("b", "B", "2"));
///
/// let vec: Vec<_> = vec
///     .into_iter()
///     .map_values(String::from)
///     .collect();
/// ```
#[cfg(doctest)]
pub struct Only2ElementTuples;

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
        vec![("A", "A"), ("B", "B"), ("C", "C")]
            .into_iter()
            .rev()
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
        vec![("a", "a"), ("b", "b"), ("c", "c")]
            .into_iter()
            .rev()
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

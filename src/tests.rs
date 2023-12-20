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

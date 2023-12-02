mod keys;
mod values;

pub use keys::FilterKeys;
pub use values::FilterValues;

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use map_vec::{Map, Set};
    use pretty_assertions::assert_eq;

    use crate::IterMap;

    #[test]
    fn hash() {
        let map = HashMap::from([
            ('a', HashSet::from([1, 2, 3])),
            ('b', HashSet::from([4, 5, 6])),
            ('c', HashSet::from([7, 8, 9])),
        ]);

        let expected = map
            .clone()
            .remove_entry(&'b')
            .into_iter()
            .collect::<HashMap<_, _>>();

        let filtered = map
            .clone()
            .into_iter()
            .filter_keys(|c| c == &'b')
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");

        let filtered = map
            .into_iter()
            .filter_values(|v| v.contains(&5))
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");
    }

    #[test]
    fn hash_ref() {
        let map = HashMap::from([
            ('a', HashSet::from([1, 2, 3])),
            ('b', HashSet::from([4, 5, 6])),
            ('c', HashSet::from([7, 8, 9])),
        ]);

        let expected = map
            .get_key_value(&'b')
            .into_iter()
            .collect::<HashMap<_, _>>();

        let filtered = map
            .iter()
            .filter_keys(|c| c == &&'b')
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");

        let filtered = map
            .iter()
            .filter_values(|v| v.contains(&5))
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");
    }

    #[test]
    fn map() {
        let map = Map::from_iter([
            ('a', Set::from_iter([1, 2, 3])),
            ('b', Set::from_iter([4, 5, 6])),
            ('c', Set::from_iter([7, 8, 9])),
        ]);

        let expected = map.get_key_value(&'b').into_iter().collect::<Map<_, _>>();

        let filtered = map
            .iter()
            .filter_keys(|c| c == &&'b')
            .collect::<Map<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");

        let filtered = map
            .iter()
            .filter_values(|v| v.contains(&5))
            .collect::<Map<_, _>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");
    }

    #[test]
    fn vec() {
        let vec = Vec::from([('a', 'A'), ('b', 'B'), ('c', 'C')]);

        let expected = vec
            .iter()
            .find(|(k, _v)| k == &'b')
            .cloned()
            .into_iter()
            .collect::<Vec<_>>();

        let filtered = vec
            .clone()
            .into_iter()
            .filter_keys(|k| k == &'b')
            .collect::<Vec<_>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");

        let filtered = vec
            .into_iter()
            .filter_values(|v| v == &'B')
            .collect::<Vec<_>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");
    }

    #[test]
    fn vec_ref() {
        let vec = Vec::from([('a', 'A'), ('b', 'B'), ('c', 'C')]);

        let expected = vec
            .iter()
            .find(|(k, _v)| k == &'b')
            .map(|(k, v)| (k, v))
            .into_iter()
            .collect::<Vec<_>>();

        let filtered = vec
            .iter()
            .map(|(k, v)| (k, v))
            .filter_keys(|k| k == &&'b')
            .collect::<Vec<_>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");

        let filtered = vec
            .iter()
            .map(|(k, v)| (k, v))
            .filter_values(|v| v == &&'B')
            .collect::<Vec<_>>();
        assert_eq!(expected, filtered);
        println!("{filtered:#?}");
    }
}

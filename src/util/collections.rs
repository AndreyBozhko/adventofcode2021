/// Macro to construct a HashSet from given elements.
///
/// # Example
/// ```
/// # use adventofcode::set;
///
/// let (a, b, c) = (1, 2, 4);
/// let s = set![0, 1, 2, a, b, c];
///
/// assert_eq!(s.len(), 4);
/// assert!(s.contains(&1));
/// assert!(!s.contains(&3));
/// ```
#[macro_export]
macro_rules! set {
    () => {{ ::std::collections::HashSet::new() }};
    ($($elem:expr),+ $(,)?) => {
        {
            let mut s = ::std::collections::HashSet::new();
            $( s.insert($elem); )+
            s
        }
    }
}

/// Macro to construct a HashMap from given key-value pairs.
///
/// # Example
/// ```
/// # use adventofcode::map;
///
/// let (a, b) = ('1', 2);
/// let m = map![a => b, 'c' => 10];
///
/// assert_eq!(m.len(), 2);
/// assert_eq!(m.get(&'1'), Some(&2));
/// assert_eq!(m.get(&'c'), Some(&10));
/// ```
#[macro_export]
macro_rules! map {
    () => {{ ::std::collections::HashMap::new() }};
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $( m.insert($k, $v); )+
            m
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_set() {
        {
            let s: HashSet<u32> = set![];
            assert!(s.is_empty());
        }
        {
            let s: HashSet<_> = set![1];
            assert_eq!(s.len(), 1);
        }
        {
            let s: HashSet<_> = set![1, 2, 3];
            assert_eq!(s.len(), 3);
        }
        {
            let s: HashSet<_> = set![1, 3, 3, 3, 7];
            assert_eq!(s.len(), 3);
        }
    }

    #[test]
    fn test_map() {
        {
            let m: HashMap<u32, u32> = map![];
            assert!(m.is_empty());
        }
        {
            let m: HashMap<_, _> = map![1 => "qux".to_string()];
            assert_eq!(m.len(), 1);
            assert_eq!(&m[&1], "qux");
        }
    }
}

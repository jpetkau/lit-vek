/*

chain:

- call stdlib chain? sure why not

*/
#[macro_export]
macro_rules! chain {
    // empty
    () => { ::std::iter::empty() };

    // ...iterable
    (...$spread:expr $(, $($tail:tt)*)?) => {
        ::std::iter::Iterator::chain(
            $spread.into_iter(), chain![ $($($tail)*)? ])
    };

    // single elem
    ($elem:expr $(, $($tail:tt)*)?) => {
        ::std::iter::Iterator::chain(
            ::std::iter::once($elem), chain!($($($tail)*)?))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chain() {
        assert_eq!(Vec::<u32>::new(), chain![].collect::<Vec<_>>());

        assert_eq!(vec![1], chain![1].collect::<Vec<_>>());
        assert_eq!(chain![1].size_hint(), (1, Some(1)));

        assert_eq!(vec![1, 2, 3], chain![1, ...[2,3]].collect::<Vec<_>>());
        assert_eq!(chain![1, ...[2,3]].size_hint(), (3, Some(3)));
    }
}

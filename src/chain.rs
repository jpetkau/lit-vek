/*

chain:

- call stdlib chain? sure why not

*/
#[macro_export]
macro_rules! chain {
    // empty
    () => { ::std::iter::empty() };

    (...$some:tt $(,)? ) => {
        ::std::iter::IntoIterator::into_iter($some)
    };

    (...$some:tt, ...$spread:expr $(, $($tail:tt)*)?) => {
        {
            let c1 = ::std::iter::IntoIterator::into_iter($some);
            let c2 = ::std::iter::Iterator::chain(c1, $spread);
            $crate::chain![...c2, $($($tail)*)?]
        }
    };

    // shift a single elem into the starting literal
    (...[$($some:tt)*], $elem:tt $(, $($tail:tt)*)?) => {
        $crate::chain![...[$($some)*, $elem], $($($tail)*)?]
    };

    // first element is a (non-literal) spread
    (...$spread:expr $(, $($tail:tt)*)?) => {
        ::std::iter::Iterator::chain(
            ::std::iter::IntoIterator::into_iter($spread),
            $crate::chain![ $($($tail)*)? ])
    };

    // shift a single elem into a new starting literal
    ($elem:expr $(, $($tail:tt)*)?) => {
        $crate::chain![...[$elem], $($($tail)*)? ]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chain() {
        assert_eq!(Vec::<u32>::new(), chain![].collect::<Vec<_>>());

        assert_eq!(vec![1], chain![1].collect::<Vec<_>>());
        assert_eq!(vec![1, 2], chain![1, 2].collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], chain![1, 2, 3].collect::<Vec<_>>());

        assert_eq!(vec![1], chain![...[1]].collect::<Vec<_>>());
        assert_eq!(vec![1, 2], chain![...[1,2]].collect::<Vec<_>>());

        assert_eq!(vec![1, 2], chain![1,...[2]].collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], chain![...[1], ...[2,3]].collect::<Vec<_>>());
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            chain![1, ...[2,3], 4, ...[], 5, ...[6]].collect::<Vec<_>>()
        );
    }
}

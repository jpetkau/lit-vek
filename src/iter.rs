/** Iterator type returned by `cycle_n()` */
#[derive(Clone, Debug)]
pub struct CycleN<I> {
    // Both `orig` and `iter` are in Option so that we don't have to
    // call into_iter() or clone() more than necessary.
    orig: Option<I>,
    iter: Option<I>,
    n: usize,
}

/** Iterator adaptor that repeats a sequence n times.

    use lit_vek::cycle_n;

    assert!(cycle_n([1, 2, 3], 2).eq([1, 2, 3, 1, 2, 3]));

The sequence must be `IntoIter`, and the iterator must be `Clone`.

This is mostly to enable the `vek![...elems, n]` syntax.
*/
pub fn cycle_n<I: IntoIterator>(it: I, n: usize) -> CycleN<I::IntoIter>
where
    I::IntoIter: Clone,
{
    match n {
        0 => CycleN {
            orig: None,
            iter: None,
            n,
        },
        1 => {
            let it = it.into_iter();
            CycleN {
                orig: None,
                iter: Some(it),
                n,
            }
        }
        _ => {
            let i1 = it.into_iter();
            let i2 = i1.clone();
            CycleN {
                orig: Some(i1),
                iter: Some(i2),
                n,
            }
        }
    }
}

impl<I: Iterator + Clone> Iterator for CycleN<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(it) = &mut self.iter {
            if let Some(x) = it.next() {
                return Some(x);
            }
            self.n -= 1;
            if self.n < 2 {
                self.iter = self.orig.take();
            } else {
                self.iter = self.orig.clone();
            }
        }
        None
    }
}

/**
Chain one more elements or iterables together into one sequence, using "spread"
syntax.

    # use {lit_vek::iter, std::array};

    let arr = [1, 2, 3];
    let vec = vec![8, 9, 10];

    assert!(
        iter![...arr, 4, ...(5..7), 7, ...vec]
        .eq([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

`iter![a, b, c]` simply returns an iterator over those elements, exactly like
`[a, b, c].into_iter()`. But elements can be prefixed with `...` to indicate
a "spread": they are chained into the resulting iterator with
`std::iter::chain()`.

See `[lit_vek::vek]` in this crate for more explanation of the syntax.

# Examples

    use {lit_vek::{iter, CycleN}, std::{array, iter}};
    let ints = [1, 2, 3];

    // Empty invocations of iter! expand to [std::iter::empty].
    let _: iter::Empty<i32> = iter![];

    // One or more arguments expand to Array::IntoIter.
    let _: array::IntoIter<_, 1> = iter![1];
    let _: array::IntoIter<_, 3> = iter![1, 2, 3];

    // The `[x; n]` syntax expands to [std::iter::repeat(x).take(n)]
    let _: iter::Take<iter::Repeat<_>> = iter![0; 5];

    // The `[...x; n]` syntax expands to [`cycle_n`] from this crate
    let _: CycleN<_> = iter![...ints; 3];

    // And chaining expands to [std::iter::Chain]
    let _: iter::Chain<_, _> = iter![...ints, ...ints];

# See also

The `vek![]` macro in this crate is a drop-in replacement for `vec![]`,
but enables the same spread syntax. It is equivalent to
`iter![].collect::<Vec<_>>()`.

The `itertools::chain!` macro is similar, except it takes only iterable
arguments rather than a mix of iterables and single elements.

```ignore
    chain![a, b, iter::once(c)] == iter![...a, ...b, c]
```
*/
#[macro_export]
macro_rules! iter {
    // empty
    () => { ::std::iter::empty() };

    // [x; n]
    ($x:tt; $n:expr) => {
        ::std::iter::repeat($x).take($n)
    };

    // [...x; n]
    (...$x:tt; $n:expr) => {
        $crate::cycle_n($x, $n)
    };

    // [...[x; n] at start
    (...[...$xs:expr; $n:expr] $($tail:tt)*) => {
        $crate::iter![...$crate::cycle_n($xs, $n) $($tail)*]
    };

    // simple array at start
    (...$some:tt $(,)? ) => {
        ::std::iter::IntoIterator::into_iter($some)
    };

    // ...[...x; n] in second item; convert to repeat_n
    (...$some:tt, ...[...$x:tt; $n:expr] $($tail:tt)*) => {
        $crate::iter![...$some, ...$crate::cycle_n($x, $n) $($tail)*]
    };

    // ...[x; n] in second item; convert to simple expr
    (...$some:tt, ...[$x:tt; $n:expr] $($tail:tt)*) => {
        $crate::iter![...$some, ...std::iter::repeat($x).take($n) $($tail)*]
    };

    // ...xs in second item and nothing else; final chain()
    (...$some:tt, ...$xs:expr $(,)?) => {
        ::std::iter::Iterator::chain(
            ::std::iter::IntoIterator::into_iter($some),
            $xs)
    };

    // ...xs in second item followed by more; continue chain()
    (...$some:tt, ...$xs:expr, $($tail:tt)*) => {
        $crate::iter![
            ... ::std::iter::Iterator::chain(
                ::std::iter::IntoIterator::into_iter($some),
                $xs),
            $($tail)*
        ]
    };

    // shift a single elem into the starting literal
    (...[$($some:tt)*], $x:tt $(, $($tail:tt)*)?) => {
        $crate::iter![...[$($some)*, $x], $($($tail)*)?]
    };

    // first element is a (non-literal) spread
    (...$xs:expr $(, $($tail:tt)*)?) => {
        ::std::iter::Iterator::chain(
            ::std::iter::IntoIterator::into_iter($xs),
            $crate::iter![ $($($tail)*)? ])
    };

    // shift a single elem into a new starting literal
    ($x:expr $(, $($tail:tt)*)?) => {
        $crate::iter![...[$x], $($($tail)*)? ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_n() {
        assert!(cycle_n(1..4, 0).eq([]));
        assert!(cycle_n(1..4, 1).eq([1, 2, 3]));
        assert!(cycle_n(1..4, 2).eq([1, 2, 3, 1, 2, 3]));

        assert!(cycle_n(0..0, 10).eq([]));
    }

    #[test]
    fn test_iter() {
        assert_eq!(Vec::<u32>::new(), iter![].collect::<Vec<_>>());

        assert_eq!(vec![1], iter![1].collect::<Vec<_>>());
        assert_eq!(vec![1, 2], iter![1, 2].collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], iter![1, 2, 3].collect::<Vec<_>>());

        assert_eq!(vec![1], iter![...[1]].collect::<Vec<_>>());
        assert_eq!(vec![1, 2], iter![...[1,2]].collect::<Vec<_>>());

        assert_eq!(vec![1, 2], iter![1,...[2]].collect::<Vec<_>>());
        assert_eq!(vec![1, 2, 3], iter![...[1], ...[2,3]].collect::<Vec<_>>());
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            iter![1, ...[2,3], 4, ...[], 5, ...[6]].collect::<Vec<_>>()
        );
    }
}

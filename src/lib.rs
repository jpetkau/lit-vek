/*!
Rust crate that defines macros to enable ES-like "spread" syntax for
literal sequences.

```rust
    # use lit_vek::{iter,vek};
    let arr = [4, 5, 6];

    // `vek!` is a drop-in replacement for `std::vec!`, except you can
    // use `...x` to expand iterables.
    assert_eq!(
        vek![1, 2, 3, ...arr, 7, 8, 9],
        [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(
        vek![1, 2, 3, ...arr, 7, 8, 9],
        [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // `iter!` provides the same syntax as iterator, similar to
    // itertools::chain()
    # use std::collections::VecDeque;
    let d: VecDeque<_> = iter![1, 2, 3, ...arr, 7, 8, 9].collect();
```
*/
mod iter;

pub use iter::{cycle_n, CycleN};

/**
A drop-in replacement for `vec![]` that adds "spread" syntax.

    # use {lit_vek::vek, std::array};

    let arr = [1, 2, 3];
    let vec = vec![8, 9, 10];

    assert_eq!(
        vek![...arr, 4, ...(5..7), 7, ...vec],
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

`vek![a, b, c]` simply returns a vec of those elements, exactly like `vec!`.
But elements can be prefixed with `...` to indicate a "spread", which is
expanded by calling `into_iter()`.

# Examples

Normal `vec!` syntax works the same way:

    use lit_vek::vek;

    assert_eq!(vek![1, 2, 3], vec![1, 2, 3]);
    assert_eq!(vek!["x"; 5], vec!["x"; 5]);

Spread syntax `[...xs]` inserts xs into the result via `into_iter()`:

    use lit_vek::vek;
    let abc = [1, 2, 3];

    assert_eq!(vek![...abc, ...abc], [1, 2, 3, 1, 2, 3]);

You can also use `[...xs; n]` to repeat a sequence n times, like
`std::iter::cycle()` but of finite length. The iterator returned
by `xs.into_iter()` must implement `Clone`.

    use lit_vek::vek;

    let abc = [1, 2, 3];
    assert_eq!(
        vek![...abc; 2], [1, 2, 3, 1, 2, 3])

And all these can be combined:

    use lit_vek::vek;
    let abc = [1, 2, 3];

    // Note that (3..5) here is an ordinary range expression,
    // not special syntax.
    assert_eq!(
        vek![1, ...[2;2], ...[...(3..5);2]],
        [1, 2, 2, 3, 4, 3, 4]);

    assert_eq!(
        vek![0, ...[...abc; 2], ...[9; 3]],
        [0, 1, 2, 3, 1, 2, 3, 9, 9, 9]);

The cycle-n-times logic is also available as a function, `cycle_n`
since it's missing from std::iterator.

# Design choices

Why `...` when most range-like things in Rust, including the existing
pattern spread syntax, use ".."? Because `..x` is already an expression
of type `std::ops::RangeTo`. The macro could still work with `..`, and
you could disambiguiate by parenthesizing if necessary:

```ignore
    let r = [..1, ..2];
    let my_ranges = vek![(..1), ..r];
```

But it seemed better to sidestep the issue.

Why prefix instead of suffix syntax, like `vek![a, bs..., c]`? Mostly
to match other languages I'm aware of with spread syntax.

# See also

The `iter![]` macro in this crate uses the same syntax, but produces
an iterator instead of a Vec. In fact `vek![...]` is  equivalent to
`iter![...].collect::<Vec<_>>()`.
*/
#[macro_export]
macro_rules! vek {
    () => { Vec::new() };

    ($($tail:tt)*) => {
        ::std::iter::Iterator::collect::<Vec<_>>($crate::iter![$($tail)*])
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let empty: Vec<u32> = vek![];
        println!("{:?}", empty);
        println!("{:?}", vek![1]);
        println!("{:?}", vek![...[1,2]]);
        println!("{:?}", vek![1, 2, 3, ...[4, 5], 6,...[7, 8]]);

        let temp = vec![2, 3, 4];
        println!("{:?}", vek![1, ...temp.iter().copied(), 5, 6]);
    }
}

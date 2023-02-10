/*!
This crate defines a few macros and utilities to enable nicer literal
syntax.
*/
mod chain;

pub use chain::{cycle_n, CycleN};

/**
A drop-in replacement for `vec![]` that adds "spread" syntax.

    # use {wec::wec, std::array};

    let arr = [1, 2, 3];
    let vec = vec![8, 9, 10];

    assert_eq!(
        wec![...arr, 4, ...(5..7), 7, ...vec],
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

`wec![a, b, c]` simply returns a vec of those elements, exactly like `vec!`.
But elements can be prefixed with `...` to indicate a "spread", which is
expanded by calling `into_iter()`.

# Examples

Normal `vec!` syntax works the same way:

    use wec::wec;

    assert_eq!(wec![1, 2, 3], vec![1, 2, 3]);
    assert_eq!(wec!["x"; 5], vec!["x"; 5]);

Spread syntax `[...xs]` inserts xs into the result via `into_iter()`:

    use wec::wec;
    let abc = [1, 2, 3];

    assert_eq!(wec![...abc, ...abc], [1, 2, 3, 1, 2, 3]);

You can also use `[...xs; n]` to repeat a sequence n times, like
`std::iter::cycle()` but of finite length. The iterator returned
by `xs.into_iter()` must implement `Clone`.

    use wec::wec;

    let abc = [1, 2, 3];
    assert_eq!(
        wec![...abc; 2], [1, 2, 3, 1, 2, 3])

And all these can be combined:

    use wec::wec;
    let abc = [1, 2, 3];

    // Note that (3..5) here is an ordinary range expression,
    // not special syntax.
    assert_eq!(
        wec![1, ...[2;2], ...[...(3..5);2]],
        [1, 2, 2, 3, 4, 3, 4]);

    assert_eq!(
        wec![0, ...[...abc; 2], ...[9; 3]],
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
    let my_ranges = wec![(..1), ..r];
```

But it seemed better to sidestep the issue.

Why prefix instead of suffix syntax, like `wec![a, bs..., c]`? Mostly
to match other languages I'm aware of with spread syntax.

# See also

The `iter![]` macro in this crate uses the same syntax, but produces
an iterator instead of a Vec. In fact `wec::wec[...]` is  equivalent to
`iter![...].collect::<Vec<_>>()`.
*/
#[macro_export]
macro_rules! wec {
    () => { Vec::new() };

    ($($tail:tt)*) => {
        ::std::iter::Iterator::collect::<Vec<_>>($crate::iter![$($tail)*])
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let empty: Vec<u32> = wec![];
        println!("{:?}", empty);
        println!("{:?}", wec![1]);
        println!("{:?}", wec![...[1,2]]);
        println!("{:?}", wec![1, 2, 3, ...[4, 5], 6,...[7, 8]]);

        let temp = vec![2, 3, 4];
        println!("{:?}", wec![1, ...temp.iter().copied(), 5, 6]);
    }
}

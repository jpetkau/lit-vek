/*
 * counting items?
 * - tricky with extend() etc
 * - recurse twice. First time builds up size and iters, second time applies them
 *
 * first pass:
 *   let iters = (elem.into_iter(), iters)
 *   let n = n + iters.0.size_hint();
 *
 *   let n = n + 1;
 *
 *   let iters = (elem.into_iter(), iters)
 *   let n = n + iters.0.size_hint();
 *
 * second pass:
 *   v = Vec::with_capacity(n);
 *   v.extend(iters.1.1.1.0);
 *   v.push(something);
 *   v.extend(iters.1.0);
 *   v.extend(iters.0);
 *   v
 *
 * this means you can't do:
 *
 *      wec![...xs.iter().cloned(), ...xs]
 *
 * though you can do:
 *
 *      wec![...xs.clone(), ...xs]
 *
 * is there some way to get the size hint without always converting to vec? 
 * probably not with the orphan rule.
 *
 */
#[macro_export]
macro_rules! wec {
    // empty
    (@pass1) => ((0,()));
    (@pass2 $v:ident $stuff:expr,) => ();

    // ...iterable
    (@pass1 ...$spread:expr $(, $($tail:tt)*)?) => {
        {
            let it = $spread.into_iter();
            let (n, its) = wec!(@pass1 $($($tail)*)?);
            (n + it.size_hint().0, (it, its))
        }
    };
    (@pass2 $v:ident $stuff:expr, ...$spread:expr $(, $($tail:tt)*)?) => {
        {
            $v.extend($stuff.0);
            wec!(@pass2 $v $stuff.1, $($($tail)*)?);
        }
    };
    // single elem
    (@pass1 $elem:expr $(, $($tail:tt)*)?) => {
        {
            let (n, its) = wec!(@pass1 $($($tail)*)?);
            (n + 1, its)
        }
    };
    (@pass2 $v:ident $stuff:expr, $elem:expr $(, $($tail:tt)*)?) => {
        {
            $v.push($elem);
            wec!(@pass2 $v $stuff, $($($tail)*)?);
        }
    };

    (@ $($tail:tt)*) => {
        compile_error!(stringify!(@ $($tail)*));
    };

    () => { Vec::new() };

    ($($tail:tt)*) => {
        {
            let (n,_its) = wec![@pass1 $($tail)*];
            let mut v = Vec::with_capacity(n);
            wec![@pass2 v _its, $($tail)*];
            v
        }
    };
}

#[macro_export]
macro_rules! wec_items {
    ($v:expr,) => ( );
    ($v:expr, ...[$($items:tt)*], $($tail:tt)*) => (
        wec_items![$v, $($items)*];
        wec_items![$v, $($tail)*];
    );
    ($v:expr, ...[$($items:tt)*]) => (
        wec_items![$v, $($items)*];
    );
    ($v:expr, ...$elem:expr, $($tail:tt)*) => ( $v.extend($elem.into_iter()); wec_items!($v, $($tail)*) );
    ($v:expr, ...$elem:expr) => ( $v.extend($elem.into_iter()); );

    ($v:expr, $elem:expr, $($tail:tt)*) => ( $v.push($elem); wec_items!($v, $($tail)*) );
    ($v:expr, $elem:expr) => ( $v.push($elem); );
}

#[macro_export]
macro_rules! wec2 {
    () => (
        Vec::new()
    );
    ($elem:expr; $n:expr) => (
        std::vec::from_elem($elem, $n)
    );
    ($($x:tt)+) => (
        {
            let mut v = Vec::new();
            wec_items![v, $($x)+];
            v
        }
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let empty : Vec<u32> = wec![];
        println!("{:?}", empty);
        println!("{:?}", wec![1]);
        println!("{:?}", wec![...[1,2]]);
        println!("{:?}", wec![1, 2, 3, ...[4, 5], 6,...[7, 8]]);
        
        let temp = vec![2,3,4];
        println!("{:?}", wec![1, ...temp.iter().copied(), 5, 6]);
    }
}

/*
counting items?
 - tricky with extend() etc
 - recurse twice. First time builds up size and iters, second time applies them

first pass:
  let iters = (elem.into_iter(), iters)
  let n = n + iters.0.size_hint();

  let n = n + 1;

  let iters = (elem.into_iter(), iters)
  let n = n + iters.0.size_hint();

second pass:
  v = Vec::with_capacity(n);
  v.extend(iters.1.1.1.0);
  v.push(something);
  v.extend(iters.1.0);
  v.extend(iters.0);
  v

this means you can't do:

  wec![...xs.iter().cloned(), ...xs]

though you can do:

  wec![...xs.clone(), ...xs]

is there some way to get the size hint without calling IntoIterator?
Don't think so.

Chain:

*/
mod dangit;

#[macro_export]
macro_rules! wec {
    () => { Vec::new() };

    ($($tail:tt)*) => {
        ::std::iter::Iterator::collect::<Vec<_>>(chain![$($tail)*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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

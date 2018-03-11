//! This library provides 'map' methods to Tuple.
//! # Example
//! ```
//! extern crate tuple_map;
//! use tuple_map::*;
//! fn main() {
//!     let (x, y) = (3, 4);
//!     let (x, y) = (x, y).map(|a| a + 5);
//!     assert_eq!(x, 8);
//!     assert_eq!(y, 9);
//! }
//! ```
//!
//! **Notes**
//! This library defines different trait depending on the length of tuple,
//! like `TupleMap1`, `TupleMap2`,..., by macro, so same docs are generated for each trait.

macro_rules! impl_tuple_map {
    ($trait: ident, $($name: ident)+ , $($item: ident)+, $($self: ident)+ , $($other: ident)+) => {
        pub trait $trait {
            type Item;

            /// Checks if every element of tuple matches a predicate, like
            /// [`std::iter::Iterator::all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all).
            /// 
            /// This method take a closure returns `true` or `false`, and tries to apply this
            /// closure to all elements of tuple. If and only if all of them returns `true`,
            /// this method return `true`.
            /// # Examples
            /// ```ignore
            /// let a = (3, 9, 12, ...);
            /// assert!(a.all(|x| x % 3 == 0));
            /// assert!(!a.all(|x| x % 4 == 0));
            /// ```
            fn all<F>(self, f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool;

            /// Checks if any element of tuple matches a predicate, like
            /// [`std::iter::Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all).
            /// 
            /// This method take a closure returns `true` or `false`, and tries to apply this
            /// closure to all elements of tuple. If any of them returns `true`,
            /// this method return `true`.
            /// # Examples
            /// ```ignore
            /// let a = (3, 9, 12, ...);
            /// assert!(a.any(|x| x % 4 == 0));
            /// assert!(!a.any(|x| x % 7 == 0));
            /// ```
            fn any<F>(self, f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool;

            /// Takes `&(a, a, a, ...)` and returns `(&a, &a, &a, ...)`
            /// # Examples
            /// ```ignore
            /// let a = (3, 3, 3, ...);
            /// assert_eq!(a.by_ref(), (&3, &3, &3, ...));
            /// ```
            fn by_ref(&self) -> ($(&Self::$item, )*);

            /// Takes `&mut (a, a, a, ...)` and returns `(&mut a, &mut a, &mut a, ...)`
            /// # Examples
            /// ```ignore
            /// let a = (3, 3, 3, ...);
            /// assert_eq!(a.by_ref(), (&mut 3, &mut 3, &mut 3, ...));
            /// ```
            fn by_ref_mut(&mut self) -> ($(&mut Self::$item, )*);

            /// Takes `&(a, a, a, ...)` and returns `(a, a, a, ...)` 
            /// # Examples
            /// ```ignore
            /// let a = (3, 3, 3, ..,);
            /// assert_eq!(a, a.clone());
            /// ```
            fn cloned(&self) -> ($(Self::$item, )*)
            where
                Self::Item: Clone
            {
                self.by_ref().map(|x| x.clone())
            }

            /// Takes a closure `f` and applies it to all elements to tuple, and produce single value.
            /// This is similar to [`std::iter::Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...)
            /// let sum = a.fold(0, |sum, x| sum + x);
            /// ```
            fn fold<B, F>(self, init: B, f: F) -> B
            where
                F: FnMut(B, Self::Item) -> B;

            /// Takes a closure `f` and applies it to all elements to tuple.
            /// `f` can cause side effect(because it's `FnMut`), but this method return nothing.
            /// Similar to [`std::iter::Iterator::for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...);
            /// let mut sum = 0;
            /// a.for_each(|x| sum += x);
            /// ```
            fn for_each<F>(self, f: F)
            where
                F: FnMut(Self::Item) -> ();

            /// Convert tuple into Vec.
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...)
            /// assert_eq(a.into_vec(), vec![3, 4, 5, ...])
            /// ```
            fn into_vec(self) -> Vec<Self::Item>;

            /// Takes a closure `f` and (a, a, a, ...), then returns (f(a), f(a), f(a), ...).
            /// Similar to [`std::iter::Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map).
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...);
            /// assert_eq!(a.map(|x| x * 2), (6, 8, 10, ...));
            /// ```
            fn map<B, F>(self, f: F) -> ($($other, )*)
            where
                F: FnMut(Self::Item) -> B;
        }
        
        impl<T> $trait for ($($self, )*) {
            type Item = T;

            fn any<F>(self, mut f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool
            {
                let ($($name,)*) = self;
                $(if f($name) { return true } )*
                false
            }
            
            fn all<F>(self, mut f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool
            {
                let ($($name,)*) = self;
                $(if !f($name) { return false } )*
                true
            }

            fn by_ref(&self) -> ($(&Self::$item, )*) {
                let ($(ref $name,)*) = *self;
                ($($name,)*)
            }

            fn by_ref_mut(&mut self) -> ($(&mut Self::$item, )*) {
                let ($(ref mut $name,)*) = *self;
                ($($name,)*)
            }
            
            fn fold<B, F>(self, mut init: B, mut f: F) -> B
            where
                F: FnMut(B, Self::Item) -> B
            {
                let ($($name,)*) = self;
                $(init = f(init, $name);)*
                init
            }
            
            fn for_each<F>(self, mut f: F)
            where
                F: FnMut(Self::Item) -> ()
            {
                let ($($name,)*) = self;
                $(f($name);)*
            }

            fn into_vec(self) -> Vec<Self::Item> {
                let ($($name,)*) = self;
                let mut v = Vec::new();
                $(v.push($name);)*
                v
            }
            
            fn map<B, F>(self, mut f: F) -> ($($other, )*)
            where
                F: FnMut(Self::Item) -> B
            {
                let ($($name,)*) = self;
                ($(f($name),)*)
            }
        }
    };
}

impl_tuple_map!{
    TupleMap1,
    a,
    Item,
    T,
    B
}
impl_tuple_map!{
    TupleMap2,
    a b,
    Item Item,
    T T,
    B B
}
impl_tuple_map!{
    TupleMap3,
    a b c,
    Item Item Item,
    T T T,
    B B B
}
impl_tuple_map!{
    TupleMap4,
    a b c d,
    Item Item Item Item,
    T T T T,
    B B B B
}
impl_tuple_map!{
    TupleMap5,
    a b c d e,
    Item Item Item Item Item,
    T T T T T,
    B B B B B
}
impl_tuple_map!{
    TupleMap6,
    a b c d e f,
    Item Item Item Item Item Item,
    T T T T T T,
    B B B B B B
}
impl_tuple_map!{
    TupleMap7,
    a b c d e f g,
    Item Item Item Item Item Item Item,
    T T T T T T T,
    B B B B B B B
}
impl_tuple_map!{
    TupleMap8,
    a b c d e f g h,
    Item Item Item Item Item Item Item Item,
    T T T T T T T T,
    B B B B B B B B
}
impl_tuple_map!{
    TupleMap9,
    a b c d e f g h i,
    Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T,
    B B B B B B B B B
}
impl_tuple_map!{
    TupleMap10,
    a b c d e f g h i j,
    Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T,
    B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap11,
    a b c d e f g h i j k,
    Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T,
    B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap12,
    a b c d e f g h i j k l,
    Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T,
    B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap13,
    a b c d e f g h i j k l m,
    Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T,
    B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap14,
    a b c d e f g h i j k l m n,
    Item Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T T,
    B B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap15,
    a b c d e f g h i j k l m n o,
    Item Item Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T T T,
    B B B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap16,
    a b c d e f g h i j k l m n o p,
    Item Item Item Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T T T T,
    B B B B B B B B B B B B B B B B
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let a = (3, 9, 12, 15);
        assert!(a.all(|x| x % 3 == 0));
        assert!(!a.all(|x| x % 4 == 0));
    }

    #[test]
    fn test_any() {
        let a = (3, 9, 12, 15);
        assert!(a.any(|x| x % 4 == 0));
        assert!(!a.any(|x| x % 7 == 0));
    }

    #[test]
    fn test_by_ref() {
        let a = (3, 3, 3);
        let b = a.by_ref();
        assert_eq!(b, (&3, &3, &3))
    }

    #[test]
    fn test_by_ref_mut() {
        let mut a = (3, 3, 3);
        assert_eq!(a.by_ref_mut(), (&mut 3, &mut 3, &mut 3));
        a.by_ref_mut().for_each(|x| *x += 5);
        assert_eq!(a, (8, 8, 8))
    }

    #[test]
    fn test_cloned() {
        let mut a = (3, 3, 3);
        assert_eq!(a, a.cloned());
        let b = a.cloned().map(|x| x * 3);
        a.by_ref_mut().for_each(|x| *x *= 3);
        assert_eq!(b, a.cloned())
    }

    #[test]
    fn test_fold() {
        let a = (3, 3, 3, 3);
        let sum = a.fold(0, |sum, x| sum + x);
        assert_eq!(sum, 12)
    }

    #[test]
    fn test_into_vec() {
        assert_eq!((3, 3, 3).into_vec(), vec![3, 3, 3]);
    }
    #[test]
    fn test_map() {
        let a = (3, 3, 3);
        let mut cnt = 0;
        let b = a.map(|x| {
            cnt += 1;
            x + cnt
        });
        assert_eq!(b, (4, 5, 6))
    }
}

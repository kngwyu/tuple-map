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
//!
//!     let v = (3, 4, 5, 6).fold(vec![], |mut v, x| {
//!         if x % 3 == 0 {
//!             v.push(x);
//!         }
//!         v
//!     });
//!     assert_eq!(v, vec![3, 6]);
//!
//!     assert!((3, 3, 3).same());
//!
//!     assert_eq!((3, 4, 5).nth(1), Some(4));
//!
//!     assert_eq!((3, 4, 5).add((1, 2, 3)), (4, 6, 8));
//!
//!     let a = (1, 2, 3);
//!     let b = ("a", "b", "c");
//!     assert_eq!(
//!         a.zipf(b, |x, y| format!("{}{}", x, y)),
//!         ("1a", "2b", "3c").map(|x| x.to_owned())
//!     );
//!
//!     assert_eq!(a.sum(), 6);
//!
//!     assert_eq!(a.tmax(), 3);
//!     assert_eq!(a.tmin(), 1);
//! }
//! ```
//!
//! **Notes**
//! This library defines different trait depending on the length of tuple,
//! like `TupleMap1`, `TupleMap2`,..., by macro, so same docs are generated for each trait.
macro_rules! impl_tuple_map {
    ($trait: ident,
     $($name_reduced: ident)*,
     $($name: ident)+,
     $($name2: ident)+,
     $($item: ident)+,
     $($self: ident)+,
     $($other: ident)+) => {
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

            /// Find the leftest element which satisfies `f` and returns it.
            /// # Example
            /// ```ignore
            /// let mut a = (3, 3, 5, 3, ...);
            /// if let Some(x) = a.by_ref_mut().find(|&&mut x| x == 5) {
            ///     *x = 3
            /// }
            /// assert!(a.same());
            /// ```
            fn find<F>(self, f: F) -> Option<Self::Item>
            where
                F: FnMut(&Self::Item) -> bool;

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

            /// return Self.
            /// It's not intended to used by user.
            fn id(self) -> ($(Self::$item,)*);
            
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

            /// return nth element in the tuple.
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ..);
            /// assert_eq!(a.nth(2), Some(5));
            /// ```
            fn nth(self, i: usize) -> Option<Self::Item>;

            /// Checks if all elements of the tuple is same.
            /// # Example
            /// ```ignore
            /// let a = (3, 3, 3, ...);
            /// assert!(a.same());
            /// ```
            fn same(self) -> bool
            where
                Self::Item: PartialEq;

            /// Takes i and checks if all elements of the tuple is same as i.
            /// # Example
            /// ```ignore
            /// let a = (3, 3, 3, ...);
            /// assert!(a.same_as(3));
            /// ```
            fn same_as(self, i: Self::Item) -> bool
            where
                Self::Item: PartialEq;

            /// Takes `(a, b, c, ...)` then returns `a + b + c ...`
            fn sum(self) -> Self::Item
            where
                 Self::Item: ::std::ops::AddAssign;

            /// Takes `(a, b, c, ...)` then returns `a + b + c ...`
            fn product(self) -> Self::Item
            where
                 Self::Item: ::std::ops::MulAssign;

            /// Takes `(a, b, c, ...)` then returns the maximum value of tuple.
            /// This method is named `tmax` instead of `max`, to avoid overlap
            /// to `std::cmp::ord::max`.
            fn tmax(self) -> Self::Item
            where
                Self::Item: ::std::cmp::PartialOrd;

            /// Takes `(a, b, c, ...)` then returns the minimum value of tuple.
            /// This method is named `tmin` instead of `min`, to avoid overlap
            /// to `std::cmp::ord::min`.
            fn tmin(self) -> Self::Item
            where
                Self::Item: ::std::cmp::PartialOrd;
            
            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)` then returns `((a, b), (a, b), (a, b), ...)` 
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...);
            /// let b = ('a', 'b', 'c', ...);
            /// assert_eq!(a.zip(b), ((3, 'a'), (4, 'b'), (5, 'c'), ...));
            /// ```
            fn zip<U, B>(self, other: U) -> ($((Self::$item, $other),)*)
            where
                U: $trait<Item = B>;

            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)` and closure f,
            /// then returns `(f(a, b), f(a, b), f(a, b), ...)` 
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...);
            /// let b = ('a', 'b', 'c', ...);
            /// assert_eq!(
            ///     a.zipf(b, |x, y| format!("{}{}", x, y)),
            ///     ("3a", "4b", "5c", ...).map(|x| x.to_owned())
            /// );
            /// ```
            fn zipf<U, I, F, B>(self, other: U, f: F) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                F: FnMut(Self::Item, I) -> B;

            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)`,
            /// then returns `(a + b, a + b, a + b, ...)` 
            /// # Example
            /// ```ignore
            /// let a = (3, 4, 5, ...);
            /// let b = (7, 8, 9, ....)
            /// assert_eq!(a.add(b), (10, 12, 14, ...));
            /// ```
            fn add<U, I, B>(self, other: U) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                Self::Item: ::std::ops::Add<I, Output = B>,
                Self: Sized,
            {
                self.zipf(other, |a, b| a + b)
            }

            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)`,
            /// then returns `(a - b, a - b, a - b, ...)` 
            /// # Example
            /// ```ignore
            /// let a = (7, 8, 9, ....);
            /// let b = (3, 4, 5, ....);
            /// assert!(a.sub(b).same());
            /// ```
            fn sub<U, I, B>(self, other: U) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                Self::Item: ::std::ops::Sub<I, Output = B>,
                Self: Sized,
            {
                self.zipf(other, |a, b| a - b)
            }

            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)`,
            /// then returns `(a * b, a * b, a * b, ...)` 
            /// # Example
            /// ```ignore
            /// let a = (7, 8, 9, ....);
            /// let b = (3, 4, 5, ....);
            /// assert_eq!(a.mul(b), (21, 32, 45, ...));
            /// ```
            fn mul<U, I, B>(self, other: U) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                Self::Item: ::std::ops::Mul<I, Output = B>,
                Self: Sized,
            {
                self.zipf(other, |a, b| a * b)
            }

            /// Takes `(a, a, a, ...)` and `(b, b, b, ...)`,
            /// then returns `(a * b, a * b, a * b, ...)` 
            /// # Example
            /// ```ignore
            /// let a = (6, 8, 10, ....);
            /// let b = (3, 4, 5, ....);
            /// assert!(a.div(b).same());
            /// ```
            fn div<U, I, B>(self, other: U) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                Self::Item: ::std::ops::Div<I, Output = B>,
                Self: Sized,
            {
                self.zipf(other, |a, b| a / b)
            }
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

            fn find<F>(self, mut f: F) -> Option<Self::Item>
            where
                F: FnMut(&Self::Item) -> bool
            {
                let ($($name,)*) = self;
                $(if f(&$name) { return Some($name) })*
                None
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

            fn id(self) -> ($(Self::$item,)*) {
                let ($($name,)*) = self;
                ($($name,)*)
            }
            
            fn into_vec(self) -> Vec<Self::Item> {
                let ($($name,)*) = self;
                let mut v = Vec::new();
                $(v.push($name);)*
                v
            }


            fn nth(self, i: usize) -> Option<Self::Item> {
                let ($($name,)*) = self;
                let mut _cnt = 0;
                $(if _cnt == i { return Some($name) } else { _cnt += 1 })*
                None
            }
            
            fn map<B, F>(self, mut f: F) -> ($($other, )*)
            where
                F: FnMut(Self::Item) -> B
            {
                let ($($name,)*) = self;
                ($(f($name),)*)
            }

            #[allow(unused_variables)]
            fn same(self) -> bool
            where
                Self::Item: PartialEq
            {
                let (first, $($name_reduced,)*) = self;
                $(if $name_reduced != first { return false } )*
                true
            }

            fn same_as(self, i: Self::Item) -> bool
            where
                Self::Item: PartialEq
            {
                let ($($name,)*) = self;
                $(if $name != i { return false })*
                true
            }

            #[allow(unused_mut)]
            fn sum(self) -> Self::Item
            where
                Self::Item: ::std::ops::AddAssign
            {
                let (mut acc, $($name_reduced,)*) = self;
                $(acc += $name_reduced;)*
                acc
            }

            #[allow(unused_mut)]
            fn product(self) -> Self::Item
            where
                Self::Item: ::std::ops::MulAssign
            {
                let (mut acc, $($name_reduced,)*) = self;
                $(acc *= $name_reduced;)*
                acc
            }

            #[allow(unused_mut)]
            fn tmax(self) -> Self::Item
            where
                Self::Item: ::std::cmp::PartialOrd
            {
                let (mut acc, $($name_reduced,)*) = self;
                $(if acc < $name_reduced {
                    acc = $name_reduced;
                })*
                acc
            }

            #[allow(unused_mut)]
            fn tmin(self) -> Self::Item
            where
                Self::Item: ::std::cmp::PartialOrd
            {
                let (mut acc, $($name_reduced,)*) = self;
                $(if acc > $name_reduced {
                    acc = $name_reduced;
                })*
                acc
            }

            fn zip<U, B>(self, other: U) -> ($((Self::$item, $other),)*)
            where
                U: $trait<Item = B>
            {
                let ($($name,)*) = self;
                let ($($name2,)*) = other.id();
                ($(($name, $name2),)*)
            }

            fn zipf<U, I, F, B>(self, other: U, mut f: F) -> ($($other,)*)
            where
                U: $trait<Item = I>,
                F: FnMut(Self::Item, I) -> B
            {
                let ($($name,)*) = self;
                let ($($name2,)*) = other.id();
                ($(f($name, $name2),)*)
            }
        }
    };
}

impl_tuple_map!{
    TupleMap1,
    ,
    a,
    a2,
    Item,
    T,
    B
}
impl_tuple_map!{
    TupleMap2,
    b,
    a b,
    a2 b2,
    Item Item,
    T T,
    B B
}
impl_tuple_map!{
    TupleMap3,
    b c,
    a b c,
    a2 b2 c2,
    Item Item Item,
    T T T,
    B B B
}
impl_tuple_map!{
    TupleMap4,
    b c d,
    a b c d,
    a2 b2 c2 d2,
    Item Item Item Item,
    T T T T,
    B B B B
}
impl_tuple_map!{
    TupleMap5,
    b c d e,
    a b c d e,
    a2 b2 c2 d2 e2,
    Item Item Item Item Item,
    T T T T T,
    B B B B B
}
impl_tuple_map!{
    TupleMap6,
    b c d e f,
    a b c d e f,
    a2 b2 c2 d2 e2 f2,
    Item Item Item Item Item Item,
    T T T T T T,
    B B B B B B
}
impl_tuple_map!{
    TupleMap7,
    b c d e f g,
    a b c d e f g,
    a2 b2 c2 d2 e2 f2 g2,
    Item Item Item Item Item Item Item,
    T T T T T T T,
    B B B B B B B
}
impl_tuple_map!{
    TupleMap8,
    b c d e f g h,
    a b c d e f g h,
    a2 b2 c2 d2 e2 f2 g2 h2,
    Item Item Item Item Item Item Item Item,
    T T T T T T T T,
    B B B B B B B B
}
impl_tuple_map!{
    TupleMap9,
    b c d e f g h i,
    a b c d e f g h i,
    a2 b2 c2 d2 e2 f2 g2 h2 i2,
    Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T,
    B B B B B B B B B
}
impl_tuple_map!{
    TupleMap10,
    b c d e f g h i j,
    a b c d e f g h i j,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2,
    Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T,
    B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap11,
    b c d e f g h i j k,
    a b c d e f g h i j k,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2,
    Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T,
    B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap12,
    b c d e f g h i j k l,
    a b c d e f g h i j k l,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2,
    Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T,
    B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap13,
    b c d e f g h i j k l m,
    a b c d e f g h i j k l m,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2 m2,
    Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T,
    B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap14,
    b c d e f g h i j k l m n,
    a b c d e f g h i j k l m n,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2 m2 n2,
    Item Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T T,
    B B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap15,
    b c d e f g h i j k l m n o,
    a b c d e f g h i j k l m n o,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2 m2 n2 o2,
    Item Item Item Item Item Item Item Item Item Item Item Item Item Item Item,
    T T T T T T T T T T T T T T T,
    B B B B B B B B B B B B B B B
}
impl_tuple_map!{
    TupleMap16,
    b c d e f g h i j k l m n o p,
    a b c d e f g h i j k l m n o p,
    a2 b2 c2 d2 e2 f2 g2 h2 i2 j2 k2 l2 m2 n2 o2 p2,
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
    fn test_find() {
        let mut a = (3, 3, 5, 3);
        if let Some(x) = a.by_ref_mut().find(|&&mut x| x == 5) {
            *x = 3
        }
        assert!(a.same());
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

    #[test]
    fn test_nth() {
        let a = (3, 4, 5, 6);
        assert_eq!(a.nth(2), Some(5));
    }

    #[test]
    fn test_same() {
        let a = (3, 3, 3);
        assert!(a.same());
    }

    #[test]
    fn test_same_as() {
        let a = (3, 3, 3);
        assert!(a.same_as(3));
    }

    #[test]
    fn test_zip() {
        let a = (1, 2, 3);
        let b = ("a", "b", "c");
        assert_eq!(a.zip(b), ((1, "a"), (2, "b"), (3, "c")));
    }

    #[test]
    fn test_zipf() {
        let a = (1, 2, 3);
        let b = ("a", "b", "c");
        assert_eq!(
            a.zipf(b, |x, y| format!("{}{}", x, y)),
            ("1a", "2b", "3c").map(|x| x.to_owned())
        );
    }

    #[test]
    fn test_add() {
        let a = (3, 4, 5);
        assert_eq!(a.add((3, 4, 5)), (6, 8, 10));
    }

    #[test]
    fn test_sub() {
        let a = (3, 4, 5);
        assert!(a.sub((1, 2, 3)).same_as(2));
    }

    #[test]
    fn test_mul() {
        let a = (3, 4, 5);
        assert_eq!(a.mul((1, 2, 3)), (3, 8, 15));
    }

    #[test]
    fn test_div() {
        let a = (6, 8, 10);
        let b = (3, 4, 5);
        assert!(a.div(b).same_as(2));
    }

    #[test]
    fn test_sum() {
        let a = (6, 8, 10);
        assert_eq!(a.sum(), 24);
    }

    #[test]
    fn test_prod() {
        let a = (6, 8, 10);
        assert_eq!(a.product(), 480);
    }

    #[test]
    fn test_tmin() {
        let a = (6, 8, 10);
        assert_eq!(a.tmin(), 6);
    }

    #[test]
    fn test_tmax() {
        let a = (6, 8, 10);
        assert_eq!(a.tmax(), 10);
    }
}

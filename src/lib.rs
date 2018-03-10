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

macro_rules! impl_tuple_map {
    ($trait: ident, $($name: ident)+ , $($item: ident)+, $($self: ident)+ , $($other: ident)+) => {
        pub trait $trait {
            type Item;

            fn all<F>(self, f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool;

            fn any<F>(self, f: F) -> bool
            where
                F: FnMut(Self::Item) -> bool;
            
            fn by_ref(&self) -> ($(&Self::$item, )*);

            fn by_ref_mut(&mut self) -> ($(&mut Self::$item, )*);

            fn cloned(&self) -> ($(Self::$item, )*)
            where
                Self::Item: Clone
            {
                self.by_ref().map(|x| x.clone())
            }
            
            fn fold<B, F>(self, init: B, f: F) -> B
            where
                F: FnMut(B, Self::Item) -> B;
            
            fn for_each<F>(self, f: F)
            where
                F: FnMut(Self::Item) -> ();
            
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
        a.by_ref_mut().for_each(|x| *x += 5);
        assert_eq!(a, (8, 8, 8))
    }

    #[test]
    fn test_cloned() {
        let mut a = (3, 3, 3);
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

# tuple-map
[![Crates.io: tuple-map](http://meritbadge.herokuapp.com/tuple-map)](https://crates.io/crates/tuple-map)
[![Build Status](https://travis-ci.org/kngwyu/tuple-map.svg?branch=master)](https://travis-ci.org/kngwyu/tuple-map)
[![Documentation](https://docs.rs/tuple-map/badge.svg)](https://docs.rs/tuple-map)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This library provides iterator-like utility methods like `map`, `fold`, `for_each`, and etc., for tuple.

# Example

``` rust
extern crate tuple_map;
use tuple_map::*;
fn main() {
    let (x, y) = (3, 4);
    let (x, y) = (x, y).map(|a| a + 5);
    assert_eq!(x, 8);
    assert_eq!(y, 9);
    
    let v = (3, 4, 5, 6).fold(vec![], |mut v, x| {
        if x % 3 == 0 {
            v.push(x);
        }
        v
    });
    assert_eq!(v, vec![3, 6]);
    
    assert!((3, 3, 3).same());
    
    assert_eq!((3, 4, 5).nth(1), Some(4));

    assert_eq!((3, 4, 5).add(1, 2, 3), (4, 6, 8));

    let a = (1, 2, 3);
    let b = ("a", "b", "c");
    assert_eq!(
        a.zipf(b, |x, y| format!("{}{}", x, y)),
        ("1a", "2b", "3c").map(|x| x.to_owned())
    );
}

```

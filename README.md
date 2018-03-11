# tuple-map
[![Crates.io: tuple-map](http://meritbadge.herokuapp.com/tuple-map)](https://crates.io/crates/tuple-map)
[![Build Status](https://travis-ci.org/kngwyu/tuple-map.svg?branch=master)](https://travis-ci.org/kngwyu/tuple-map)
[![Documentation](https://docs.rs/tuple_map/badge.svg)](https://docs.rs/tuple_map)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This library provides iterator-like utilities for `map`, `fold`, `for_each`, and etc., for tuple.

# Example

``` rust
extern crate tuple_map;
use tuple_map::*;
fn main() {
    let (x, y) = (3, 4);
    let (x, y) = (x, y).map(|a| a + 5);
    assert_eq!(x, 8);
    assert_eq!(y, 9);
}

```

# `tor.rs`

A reimplementation of the `vec!` macro using syntax extensions.

`tor!($SOMETHING)` expands into:

``` rust
{
    use std::slice::BoxedSlice;
    use std::boxed::HEAP;
    let xs = box (HEAP) [$SOMETHING];
    xs.into_vec()
}
```

For the implementation details, see the `expand_tor` function in the
[src/lib.rs](/src/lib.rs) file.

You can find a rather minimal test suite in the [tests/tor.rs](/tests/tor.rs)
file.

# License

tor.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

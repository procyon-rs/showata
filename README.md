# evcxr_displayers

A library of displayers (and helpers) for [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter).

Feedbacks (comments, tickets, PR,...) are welcomes.

## Usage

Inside your jupyter (rust) notebook:

```txt
:dep evcxr_displayers = { version = "0.1.0" features=["display_nalgebra"]}

extern crate evcxr_displayers;
use evcxr_displayers::EvcxrDisplay;
```

![Sample Screenshot](https://i.imgur.com/HH3qUXh.png)

### Notes

Currently the project groups displayers as `features` instead of packages. But it could change in the futures if use of package is more usefull (than just "it's the recommanded way in the cargo doc").

Why features (vs packages):

- less lines of code to insert into a notebook (but longer)
- ability to share a `trait`and provide `ìmpl` for external crates (else need to create one trait `<Package>Display` per package and to call `use <package>::<Package>Display` for each package in the notebook)

## Links

- [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter) A Jupyter Kernel for the Rust programming language.
- [nalgebra](https://crates.io/crates/nalgebra) A linear algebra library for the Rust programming language.

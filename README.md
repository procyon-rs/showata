# showata

[![license](https://img.shields.io/crates/l/showata.svg)](https://spdx.org/licenses/Apache-2.0.html)
[![version](https://img.shields.io/crates/v/showata.svg)](https://crates.io/crates/showata)
[![Release Doc](https://docs.rs/showata/badge.svg)](https://docs.rs/showata)

[![Actions Status](https://github.com/procyon-rs/showata/workflows/ci-flow/badge.svg)](https://github.com/procyon-rs/showata/actions)


A library of to show data (in browser, [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter)) as table, chart....

The crate provides display for:

- image
- vector and slice (as table)
- ndarray's Array (as table)
- nalgebra's matrix (as table)

Feedbacks (comments, tickets, PR,...) are welcomes.

## Usages

### Inside Jupyter

Inside your jupyter (rust) notebook:

```rust
:dep showata = { version = "0.3.0" features=["show_ndarray"]}
:dep ndarray = "0.14"

use showata::Showable;
use ndarray::Array2;

let data: Array2 = Array2::<f64>::zeros((3, 4));
data.show()
```

### Inside Editor

Inside your favorite editor/IDE:

```rust
use showata::Showable;
use ndarray::Array2;

let data = Array2::<f64>::zeros((3, 4));
data.show().unwrap();
```

```sh
# By default the data will be shown inside your web browser.
cargo run

# Show nothing
SHOWATA_MEDIUM=Noop cargo run
```

### Show Chart

see [vega_lite_3](https://github.com/procyon-rs/vega_lite_3.rs)

### Notes

Currently the project groups showers as `features` instead of packages. But it could change in the futures if use of package is more useful (than just "it's the recommended way in the cargo doc").

Why features (vs packages):

- less lines of code to insert into a notebook (but longer)
- ability to share a `trait`and provide `impl` for external crates

## Links

- [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter) A Jupyter Kernel for the Rust programming language.
- [nalgebra](https://crates.io/crates/nalgebra) A linear algebra library for the Rust programming language.
- [Wiki - AGuideToRustGraphicsLibraries2019](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)
- [A Dramatic Tour through Python’s Data Visualization Landscape (including ggplot and Altair) – Regress to Impress](https://dsaber.com/2016/10/02/a-dramatic-tour-through-pythons-data-visualization-landscape-including-ggplot-and-altair/)
- [Specifying Data in Altair — Altair 3.0.0 documentation](https://altair-viz.github.io/user_guide/data.html#long-form-vs-wide-form-data)
- [Visualization — list of Rust libraries/crates // Lib.rs](https://lib.rs/visualization)
- [Quicktype](https://quicktype.io/) (got issue with the alternative https://transform.now.sh/json-to-rust-serde) was used to bootstrap `vegalite.rs` from the [vega-lite's json schema](https://vega.github.io/schema/vega-lite/v3.json)

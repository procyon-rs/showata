# showata

A library of to show data (in browser, [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter)) as table, chart....

Feedbacks (comments, tickets, PR,...) are welcomes.

## Usages

### Inside Jupyter

Inside your jupyter (rust) notebook:

```txt
:dep showata = { version = "0.1.0" features=["show_nalgebra"]}

extern crate showata;
use showata::Showable;

let data:  = 
```

![Sample Screenshot](https://i.imgur.com/HH3qUXh.png)

### Show Chart

```rust
        let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv")).unwrap();
        let values = rdr.deserialize().into_iter().collect::<Result<Vec<Item>, csv::Error>>().unwrap();
        let chart = VegaliteBuilder::default()
            // .title(Some(crate::vegalite::Title::String("Hello".to_owned())))
            // .width(400.0)
            // .height(200.0)
            // .padding(Some(Padding::Double(5.0)))
            .description("Google's stock price over time.".to_owned())
            .data(Some((&values).into()))
            .transform(Some(vec![
                TransformBuilder::default().filter(Some(
                    PurpleLogicalOperandPredicate::String("datum.symbol==='GOOG'".to_owned())
                )).build().unwrap()
            ]))
            .mark(Some(AnyMark::Enum(Mark::Line)))
            .encoding(Some(EncodingBuilder::default()
                .x(XClassBuilder::default().field(Some(Field::String("date".to_string()))).def_type(Some(Type::Temporal)).build().unwrap())
                .y(XClassBuilder::default().field(Some(Field::String("price".to_string()))).def_type(Some(Type::Quantitative)).build().unwrap())
                .build()
                .unwrap()
            ))
            .build()
            .unwrap();
        chart.show().unwrap();
```

Currently, [Vega-Lite v3](https://vega.github.io/vega-lite/) is the only way to show chart. The rust API to use `vega-lite` will be improve in the future (and maybe moved into it's own crate).

### Notes

Currently the project groups showers as `features` instead of packages. But it could change in the futures if use of package is more useful (than just "it's the recommended way in the cargo doc").

Why features (vs packages):

- less lines of code to insert into a notebook (but longer)
- ability to share a `trait`and provide `impl` for external crates (else need to create one trait `<Package>Display` per package and to call `use <package>::<Package>Display` for each package in the notebook)

## Links

- [evcxr_jupyter](https://crates.io/crates/evcxr_jupyter) A Jupyter Kernel for the Rust programming language.
- [nalgebra](https://crates.io/crates/nalgebra) A linear algebra library for the Rust programming language.
- [Wiki - AGuideToRustGraphicsLibraries2019](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)
- [A Dramatic Tour through Python’s Data Visualization Landscape (including ggplot and Altair) – Regress to Impress](https://dsaber.com/2016/10/02/a-dramatic-tour-through-pythons-data-visualization-landscape-including-ggplot-and-altair/)
- [Specifying Data in Altair — Altair 3.0.0 documentation](https://altair-viz.github.io/user_guide/data.html#long-form-vs-wide-form-data)
- [Visualization — list of Rust libraries/crates // Lib.rs](https://lib.rs/visualization)
- [Quicktype](https://quicktype.io/) (got issue with the alternative https://transform.now.sh/json-to-rust-serde) was used to bootstrap vegalite.rs from the [vega-lite's json schema](https://vega.github.io/schema/vega-lite/v3.json)

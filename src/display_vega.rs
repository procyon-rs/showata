// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//use vega_lite::Spec;
use crate::vegalite::Vegalite;
use serde::Serialize;
use serde_json;
use crate::vegalite;
use crate::ContentInfo;
use crate::Displayable;
use failure::Error;

impl Displayable for Vegalite {
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        /// vega3-extension 	application/vnd.vega.v3+json, application/vnd.vegalite.v2+json 	.vg, .vl, .vg.json, .vl.json, .vega, .vegalite
        let content = serde_json::to_string(self)?;
        Ok(ContentInfo{
            content,
            mime_type: "application/vnd.vegalite.v2+json".into(),
        })
    }

    // TODO for html use [vega/vega-embed: Publish Vega visualizations as embedded web components with interactive parameters.](https://github.com/vega/vega-embed)
    // TODO add an Config parameter (with config for to json str, config for embed)
    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = VEGA_EMBED_HTML_TEMPLATE.replace("{{ spec_as_json }}", &dod.content);
        Ok(content.into())
    }

}

const VEGA_EMBED_HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <!-- Import Vega 5 & Vega-Lite 3 (does not have to be from CDN) -->
  <script src="https://cdn.jsdelivr.net/npm/vega@5"></script>
  <script src="https://cdn.jsdelivr.net/npm/vega-lite@3"></script>
  <!-- Import vega-embed -->
  <script src="https://cdn.jsdelivr.net/npm/vega-embed@4"></script>
</head>
<body>

<div id="vis"></div>

<script type="text/javascript">
  var spec = {{ spec_as_json }};
  vegaEmbed('#vis', spec).then(function(result) {
    // Access the Vega view instance (https://vega.github.io/vega/docs/api/view/) as result.view
  }).catch(console.error);
</script>
</body>
</html>
"#;

pub fn iter_to_data<T>(v: impl Iterator<Item = T>) -> vegalite::Data
where
    T: Serialize,
{
    vegalite::Data {
        format: None,
        name: None,
        url: None,
        values: Some(iter_to_data_inline_dataset(v))
    }
}

fn iter_to_data_inline_dataset<T>(v: impl Iterator<Item = T>) -> vegalite::DataInlineDataset
where
    T: Serialize,
{
    // let values = v.map(|it|{
    //     match serde_json.to_json(it) {
    //         v: bool => InlineDataset::Bool(v),
    //         v: Double(f64),
    // String(String),

    //         v => AnythingMap(HashMap<String, Option<serde_json::Value>>),
    //     }
    // })
    let values = v.map(|it|
        serde_json::to_value(it)
    ).collect::<Result<Vec<_>, _>>().expect("TODO manage error in iter_to_dataInlineDataSet");
    vegalite::DataInlineDataset::UnionArray(values)
}

impl<T> From<&[T]> for vegalite::Data
where
    T: Serialize,
{
    fn from(v: &[T]) -> Self {
        iter_to_data(v.iter())
    }
}

impl<T> From<&Vec<T>> for vegalite::Data
where
    T: Serialize,
{
    fn from(v: &Vec<T>) -> Self {
        iter_to_data(v.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vegalite::*;
// {
//   "$schema": "https://vega.github.io/schema/vega-lite/v3.json",
//   "description": "Google's stock price over time.",
//   "data": {"url": "data/stocks.csv"},
//   "transform": [{"filter": "datum.symbol==='GOOG'"}],
//   "mark": "line",
//   "encoding": {
//     "x": {"field": "date", "type": "temporal"},
//     "y": {"field": "price", "type": "quantitative"}
//   }
// }

    #[derive(Serialize)]
    pub struct Item {
        pub symbol: String,
        pub date: String,
        pub price: f64,
    }

    #[test]
    fn test_save_as() {
        //let values = vec![1,2,3,4,5,6];
        let values = vec![
            Item{
                symbol: "MSFT".to_owned(),
                date: "Jan 1 2000".to_owned(),
                price: 39.81_f64,
            },
            Item{
                symbol: "MSFT".to_owned(),
                date: "Feb 1 2000".to_owned(),
                price: 36.35_f64,
            },
            Item{
                symbol: "MSFT".to_owned(),
                date: "Mar 1 2000".to_owned(),
                price: 43.22_f64,
            },
        ];
        //let data = iter_to_data(values.iter());
        let chart = VegaliteBuilder::default()
            .title(Some(crate::vegalite::Title::String("Hello".to_owned())))
            .width(400.0)
            .height(200.0)
            //.padding(5.0)
            .data(Some((&values).into()))
            // .data(
            //     DataBuilder::default()
            //         .url(Some(
            //             // "https://vega.github.io/vega-lite/data/flare.json".to_owned(),
            //             format!("{}/examples/res/data/flare.json", std::env::current_dir().unwrap().display()),
            //         ))
            //         .build()
            //         .unwrap(),
            // )
            .mark(Some(AnyMark::Enum(Mark::Line)))
            .encoding(Some(EncodingBuilder::default()
                .x(XClassBuilder::default().field(Some(Field::String("date".to_string()))).def_type(Some(Type::Temporal)).build().unwrap())
                .y(XClassBuilder::default().field(Some(Field::String("price".to_string()))).def_type(Some(Type::Quantitative)).build().unwrap())
                .build()
                .unwrap()
            ))
            .build()
            .unwrap();
        chart.display().unwrap();
    }

    // impl From<String> for crate::vegalite::Title {
    //     fn from(v: String) -> Self {
    //         crate::vegalite::Title::String(v)
    //     }
    // }

    impl<S> From<S> for crate::vegalite::Title
    where
        S: Into<String>,
    {
        fn from(v: S) -> Self {
            crate::vegalite::Title::String(v.into())
        }
    }

    // impl<S> From<S> for Option<crate::vegalite::Title> where S: Into<String> {
    //     fn from(v: S) -> Self {
    //         Some(crate::vegalite::Title::String(v.into()))
    //     }
    // }

    // impl<'a> From<&'a str> for Option<crate::vegalite::Title> {
    //     fn from(v: &'a str) -> Self {
    //         Some(crate::vegalite::Title::String(v.into()))
    //     }
    // }

    // impl<'a> From<&'a str> for crate::vegalite::Title {
    //     fn from(v: &'a str) -> Self {
    //         crate::vegalite::Title::String(v.into())
    //     }
    // }
}

#[cfg(test)]
mod tests_typed_builder {

    use typed_builder::TypedBuilder;

    #[derive(PartialEq, Debug, TypedBuilder)]
    struct Foo {
        #[builder(default)]
        s: Option<String>,

        // Mandatory Field:
        x: i32,

        // #[default] without parameter - use the type's default
        #[builder(default)]
        y: Option<i32>,

        // Or you can set the default
        #[builder(default = 20)]
        z: i32,

        // If the default cannot be parsed, you must encode it as a string
        #[builder(default_code = "vec![30, 40]")]
        w: Vec<u32>,
    }

    #[test]
    fn test_typed_builder() {
        assert_eq!(
            Foo::builder().x(1).y(2).z(3).w(vec![4, 5]).build(),
            Foo {
                s: None,
                x: 1,
                y: Some(2),
                z: 3,
                w: vec![4, 5]
            }
        );

        // Change the order of construction:
        assert_eq!(
            Foo::builder().z(1).x(2).w(vec![4, 5]).y(3).build(),
            Foo {
                s: None,
                x: 2,
                y: Some(3),
                z: 1,
                w: vec![4, 5]
            }
        );

        // Optional fields are optional:
        assert_eq!(
            Foo::builder().x(1).build(),
            Foo {
                s: None,
                x: 1,
                y: None,
                z: 20,
                w: vec![30, 40]
            }
        );

        // assert_eq!(
        //     Foo::builder().s("hello").x(1).build()
        //     , Foo {s: Some("hello".into()), x: 1, y: None, z: 20, w: vec![30, 40] });

        // This will not compile - because we did not set x:
        //Foo::builder().build();

        // This will not compile - because we set y twice:
        //Foo::builder().x(1).y(2).y(3);
    }
}

#[cfg(test)]
mod tests_builder {

    use derive_builder::Builder;

    #[derive(PartialEq, Debug, Clone, Builder)]
    #[builder(setter(into))]
    struct Foo {
        // #[default] without parameter - use the type's default
        #[builder(default)]
        s: Option<String>,

        // Mandatory Field:
        x: i32,

        // #[default] without parameter - use the type's default
        #[builder(default)]
        y: Option<i32>,

        // Or you can set the default
        #[builder(default = "20")]
        z: i32,

        // If the default cannot be parsed, you must encode it as a string
        #[builder(default = "vec![30, 40]")]
        w: Vec<u32>,
    }

    #[test]
    fn test_typed_builder() {
        assert_eq!(
            FooBuilder::default()
                .x(1)
                .y(2)
                .z(3)
                .w(vec![4, 5])
                .build()
                .unwrap(),
            Foo {
                s: None,
                x: 1,
                y: Some(2),
                z: 3,
                w: vec![4, 5]
            }
        );

        // Change the order of construction:
        assert_eq!(
            FooBuilder::default()
                .z(1)
                .x(2)
                .w(vec![4, 5])
                .y(3)
                .build()
                .unwrap(),
            Foo {
                s: None,
                x: 2,
                y: Some(3),
                z: 1,
                w: vec![4, 5]
            }
        );

        // Optional fields are optional:
        assert_eq!(
            FooBuilder::default().x(1).build().unwrap(),
            Foo {
                s: None,
                x: 1,
                y: None,
                z: 20,
                w: vec![30, 40]
            }
        );

        // Optional + into:
        assert_eq!(
            FooBuilder::default()
                .s("hello".to_owned())
                .x(1)
                .build()
                .unwrap(),
            Foo {
                s: Some("hello".into()),
                x: 1,
                y: None,
                z: 20,
                w: vec![30, 40]
            }
        );

        // This will not compile - because we did not set x:
        //Foo::builder().build();

        // This will not compile - because we set y twice:
        //Foo::builder().x(1).y(2).y(3);
    }
}

// #[cfg(test)]
// mod tests_struct_default {

//     #[derive(PartialEq, Debug, Clone, Default)]
//     struct Foo {
//         s: Option<String>,
//         x: i32,
//         y: Option<i32>,
//         z: i32,
//         w: Vec<u32>,
//     }

//     #[test]
//     fn tests_struct_default() {
//         assert_eq!(
//             Foo{
//                 x: 1,
//                 y: 2,
//                 z: 3,
//                 w: vec![4, 5],
//                 ..Default::default()
//             }
//             , Foo {s: None, x: 1, y: Some(2), z: 3, w: vec![4, 5] });

//         // Change the order of construction:
//         assert_eq!(
//             Foo{
//                 z: 1,
//                 x: 2,
//                 w: vec![4, 5],
//                 y: 3,
//                 ..Default::default()
//             }
//             , Foo {s: None, x: 2, y: Some(3), z: 1, w: vec![4, 5] });

//         // Optional fields are optional:
//         assert_eq!(
//             Foo{
//                 x: 1,
//                 ..Default::default()
//             }
//             , Foo {s: None, x: 1, y: None, z: 0, w: vec![] });

//         // Optional + into:
//         assert_eq!(
//             Foo{
//                 s: "hello".to_owned(),
//                 x: 1,
//                 ..Default::default()
//             }
//             , Foo {s: Some("hello".into()), x: 1, y: None, z: 20, w: vec![30, 40] });

//         // This will not compile - because we did not set x:
//         //Foo::builder().build();

//         // This will not compile - because we set y twice:
//         //Foo::builder().x(1).y(2).y(3);
//     }
// }

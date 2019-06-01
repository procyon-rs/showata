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
        // vega3-extension: .vg, .vl, .vg.json, .vl.json, .vega, .vegalite
        // mime-types: 	application/vnd.vega.v3+json, application/vnd.vegalite.v2+json
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
    use csv;
    use std::path::Path;
    use serde::{Serialize, Deserialize};

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

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        pub symbol: String,
        pub date: String,
        pub price: f64,
    }

    #[test]
    fn test_save_as() {
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

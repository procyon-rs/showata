use showata::vegalite3::*;
use showata::Showable;
use csv;
use std::path::Path;
use serde::{Serialize, Deserialize};
use failure::Error;


#[derive(Serialize, Deserialize)]
pub struct Item {
    pub symbol: String,
    pub date: String,
    pub price: f64,
}

fn main() -> Result<(), Error> {
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

    let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;
    let values = rdr.deserialize().into_iter().collect::<Result<Vec<Item>, csv::Error>>()?;
    let chart = VegaliteBuilder::default()
        // .title(Some(crate::vegalite::Title::String("Hello".to_owned())))
        // .width(400.0)
        // .height(200.0)
        // .padding(Some(Padding::Double(5.0)))
        .description("Google's stock price over time.")
        .data(&values)
        .transform(vec![
            TransformBuilder::default().filter(
                "datum.symbol==='GOOG'"
            ).build().unwrap()
        ])
        .mark(Mark::Line)
        .encoding(EncodingBuilder::default()
            .x(XClassBuilder::default().field("date").def_type(StandardType::Temporal).build().unwrap())
            .y(YClassBuilder::default().field("price").def_type(StandardType::Quantitative).build().unwrap())
            .build().unwrap()
        )
        .build().unwrap();
    chart.show()?;
    eprint!("shown");
    Ok(())
}

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
use crate::ContentInfo;
use crate::Showable;
use anyhow::Error;
use ndarray;

impl<A, D> Showable for ndarray::Array<A, D>
where
    D: ndarray::Dimension,
    A: std::fmt::Debug,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        let v = self;
        let col_size = match v.ndim() {
            1 => v.shape()[0],
            2 => v.shape()[1],
            n => unimplemented!("only support 1D or 2D array not {}D", n),
        };
        let mut html = String::new();
        html.push_str("<table>");
        html.push_str("<tr>");
        html.push_str("<th></th>");
        for c in 0..col_size {
            html.push_str(&format!("<th>{}</th>", c));
        }
        html.push_str("</tr>");
        for (r, row) in v.genrows().into_iter().enumerate() {
            html.push_str("<tr>");
            html.push_str(&format!("<th>{}</th>", r));
            for v in row.iter() {
                html.push_str(&format!("<td>{:?}</td>", v));
            }
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        Ok(ContentInfo {
            mime_type: "text/html".into(),
            content: html,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Showable;

    #[test]
    fn test_no_crash_on_3x4() {
        use ndarray::Array2;
        let m = Array2::<f64>::zeros((3, 4));
        m.show().unwrap();
    }

    #[test]
    fn test_no_crash_on_3x1() {
        use ndarray::Array1;
        let m = Array1::<f64>::zeros(3);
        m.show().unwrap();
    }
}

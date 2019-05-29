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
use crate::Displayable;
use nalgebra as na;
use num_traits;
use failure::Error;

impl<N, R, C, S> Displayable for na::Matrix<N, R, C, S> where
    N: na::Scalar + PartialOrd + num_traits::sign::Signed,
    R: na::Dim,
    C: na::Dim,
    S: na::base::storage::Storage<N, R, C>,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        let v = self;
        let (row_size, col_size) = v.shape();
        let mut html = String::new();
        html.push_str("<table>");
        html.push_str("<tr>");
        html.push_str("<th></th>");
        for c in 0..col_size {
            html.push_str(&format!("<th>{:?}</th>", c));
        }
        html.push_str("</tr>");
        unsafe {
            for r in 0..row_size {
                html.push_str("<tr>");
                html.push_str(&format!("<th>{:?}</th>", r));
                for c in 0..col_size {
                    html.push_str(&format!("<td>{:?}</td>", v.get_unchecked(r, c)));
                }
                html.push_str("</tr>");
            }
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
    use super::*;
    use crate::Displayable;

    #[test]
    fn test_no_crash_on_3x4() {
        let m = na::Matrix3x4::new(11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34);
        m.display().unwrap();
        m.evcxr_display();
    }
}

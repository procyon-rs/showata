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
use nalgebra as na;
use num_traits;

impl<N: na::Scalar + PartialOrd + num_traits::sign::Signed, R: na::Dim, C: na::Dim, S: na::base::storage::Storage<N, R, C>> crate::EvcxrDisplay for na::Matrix<N, R, C, S> {
    fn evcxr_display(&self) {
        let (row_size, col_size) = self.shape();
        let mut html = String::new();
        html.push_str("<table>");
        unsafe{
            html.push_str("<tr>");
            html.push_str("<th></th>");
            for c in 0..col_size {
                html.push_str(&format!("<th>{:?}</th>", c));
            }
            html.push_str("<tr>");
            for r in 0..row_size {
                html.push_str("<tr>");
                html.push_str(&format!("<th>{:?}</th>", r));
                for c in 0..col_size {
                    html.push_str(&format!("<td>{:?}</td>", self.get_unchecked(r, c)));
                }
                html.push_str("</tr>");
            }
        }
        html.push_str("</table>");
        crate::display_text("text/html", html);
    }
}

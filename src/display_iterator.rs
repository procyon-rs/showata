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

pub fn display_iter<V, T>(v: V) where
    V: Iterator<Item = T>,
    T: std::fmt::Debug,
 {
        let mut html = String::new();
        html.push_str("<table>");
        html.push_str("<tr>");
        html.push_str("<th></th><th>0</th>");
        html.push_str("<tr>");
        for (r, row) in v.enumerate() {
            html.push_str("<tr>");
            html.push_str(&format!("<th>{}</th><td>{:?}</td>", r, row));
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        crate::display_text("text/html", html);
}

impl<T> crate::EvcxrDisplay for Vec<T>
where
    T: std::fmt::Debug,
{
    fn evcxr_display(&self) {
        display_iter(self.iter())
    }
}

impl<T> crate::EvcxrDisplay for &[T]
where
    T: std::fmt::Debug,
{
    fn evcxr_display(&self) {
        display_iter(self.iter())
    }
}

// impl<T> crate::EvcxrDisplay for IntoIterator<Item=T, IntoIter=Iterator<Item=T>>
// where
//     T: std::fmt::Debug,
// {
//     fn evcxr_display(&self) {
//         display_iter(self.into_iter())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::EvcxrDisplay;

    #[test]
    fn test_vec() {
        let m = vec![1.0, 2.0, 3.0];
        m.evcxr_display();
    }

    #[test]
    fn test_slice() {
        let m = vec![1.0, 2.0, 3.0];
        m.as_slice().evcxr_display();
    }
}

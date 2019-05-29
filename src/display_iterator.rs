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
use failure::Error;
use crate::ContentInfo;
use crate::Displayable;

pub fn from_iter<T>(v: impl Iterator<Item = T>) -> ContentInfo
where
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
    ContentInfo {
        mime_type: "text/html".into(),
        content: html,
    }
}

impl<T> Displayable for Vec<T> where
    T: std::fmt::Debug,{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        Ok(from_iter(self.iter()))
    }
}

impl<T> Displayable for &[T] where
    T: std::fmt::Debug,{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        Ok(from_iter(self.iter()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let m = vec![1.0, 2.0, 3.0];
        //TODO test content of m
        m.display().unwrap();
    }

    #[test]
    fn test_slice() {
        let m0 = vec![1.0, 2.0, 3.0];
        let m = m0.as_slice();
        //TODO test content of m
        m.display().unwrap();
    }
}

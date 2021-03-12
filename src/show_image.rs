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
use anyhow::{anyhow, Error};
use base64;
use image;
use std::ops::Deref;

impl Showable for image::DynamicImage {
    fn to_content_info<'a>(&'a self) -> Result<ContentInfo, Error> {
        self.as_rgba8()
            .ok_or(anyhow!("failed to view the image as rbga"))
            .and_then(|i| i.to_content_info())
    }

    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = IMAGE_EMBED_HTML_TEMPLATE
            .replace("{{ content }}", &dod.content)
            .replace("{{ mime_type }}", &dod.mime_type);
        Ok(content.into())
    }
}

impl<P, Container> Showable for image::ImageBuffer<P, Container>
where
    P: image::Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [u8]>,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        let mut buffer = Vec::new();
        image::png::PngEncoder::new(&mut buffer).encode(
            self,
            self.width(),
            self.height(),
            <P as image::Pixel>::COLOR_TYPE,
        )?;
        Ok(ContentInfo {
            content: base64::encode(&buffer),
            mime_type: "image/png;base64".into(),
        })
    }

    fn to_html_page<'a>(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = IMAGE_EMBED_HTML_TEMPLATE
            .replace("{{ content }}", &dod.content)
            .replace("{{ mime_type }}", &dod.mime_type);
        Ok(content.into())
    }
}

const IMAGE_EMBED_HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
</head>
<body>

<div id="vis">
<img src="data:{{ mime_type }}, {{ content }}"/>
</div>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Showable;
    use image;

    #[test]
    fn test_show_local_image() {
        let img = image::open("examples/res/rustacean-flat-happy.png").unwrap();
        img.show().unwrap();
    }

    #[test]
    fn test_show_gen_image() {
        let img = image::ImageBuffer::from_fn(256, 256, |x, y| {
            if (x as i32 - y as i32).abs() < 3 {
                image::Rgb([0, 0, 255])
            } else {
                image::Rgb([0, 0, 0])
            }
        });
        img.show().unwrap();
    }
}

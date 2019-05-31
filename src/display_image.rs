use image;
use std::ops::Deref;
use base64;
use failure::Error;
use failure::format_err;
use crate::ContentInfo;
use crate::Displayable;

// impl EvcxrResult for image::RgbImage {
//     fn evcxr_display(&self) {
//         let mut buffer = Vec::new();
//         image::png::PNGEncoder::new(&mut buffer).encode(&**self, self.width(), self.height(),
//             image::ColorType::RGB(8)).unwrap();
//         let img = base64::encode(&buffer);
//         println!("EVCXR_BEGIN_CONTENT image/png\n{}\nEVCXR_END_CONTENT", img);
//     }
// }
// impl EvcxrResult for image::GrayImage {
//     fn evcxr_display(&self) {
//         let mut buffer = Vec::new();
//         image::png::PNGEncoder::new(&mut buffer).encode(&**self, self.width(), self.height(),
//             image::ColorType::Gray(8)).unwrap();
//         let img = base64::encode(&buffer);
//         println!("EVCXR_BEGIN_CONTENT image/png\n{}\nEVCXR_END_CONTENT", img);
//     }
// }

impl Displayable for image::DynamicImage {
    fn to_content_info<'a>(&'a self) -> Result<ContentInfo, Error> {
        self.as_rgba8().ok_or(format_err!("failed to view the image as rbga")).and_then(|i| i.to_content_info())
    }

    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = IMAGE_EMBED_HTML_TEMPLATE.replace("{{ content }}", &dod.content)
        .replace("{{ mime_type }}", &dod.mime_type);
        Ok(content.into())
    }
}

impl<P, Container> Displayable for image::ImageBuffer<P, Container> where
    P: image::Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [u8]>,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        let mut buffer = Vec::new();
        image::png::PNGEncoder::new(&mut buffer).encode(self, self.width(), self.height(), <P as image::Pixel>::color_type())?;
        Ok(ContentInfo{
            content: base64::encode(&buffer),
            mime_type: "image/png;base64".into(),
        })
    }

    fn to_html_page<'a>(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = IMAGE_EMBED_HTML_TEMPLATE.replace("{{ content }}", &dod.content)
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
    use image;
    use crate::Displayable;

    #[test]
    fn test_display_local_image() {
        let img = image::open("examples/res/rustacean-flat-happy.png").unwrap();
        img.display().unwrap();
    }

    #[test]
    fn test_display_gen_image() {
        let img = image::ImageBuffer::from_fn(256, 256, |x, y| {
            if (x as i32 - y as i32).abs() < 3 {
                image::Rgb([0, 0, 255])
            } else {
                image::Rgb([0, 0, 0])
            }
        });
        img.display().unwrap();
    }
}

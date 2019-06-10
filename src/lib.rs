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
mod show_iterator;

#[cfg(feature = "show_nalgebra")]
mod show_nalgebra;

#[cfg(feature = "show_ndarray")]
mod show_ndarray;

#[cfg(feature = "show_image")]
mod show_image;

use std::path::Path;
use failure::Error;
use std::time::SystemTime;

pub struct ContentInfo {
    pub mime_type: String,
    pub content: String,
}

pub trait Showable {
    fn to_content_info(&self) -> Result<ContentInfo, Error>;

    // the name of this function is hardcoded by evcxr
    fn evcxr_display(&self) {
        let ci = self.to_content_info().expect("to be convertible into ContentInfo");
        show_text_in_jupyter(ci.mime_type, ci.content);
    }

    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = CONTENT_EMBED_HTML_TEMPLATE.replace("{{ content }}", &dod.content);
        Ok(content.into())
    }

    fn show_in_browser(&self) -> Result<(), Error> {
        let mut dir = std::env::temp_dir();
        dir.push(env!("CARGO_PKG_NAME"));
        //TODO security check that the user can read/write only his own files
        std::fs::create_dir_all(&dir)?;
        //TODO generate random/timestamp file
        let epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let path = dir.join(format!("show-{}.html", epoch.as_nanos()));
        let html = self.to_html_page()?;
        std::fs::write(&path, html)?;
        opener::open(path.as_os_str())?;
        Ok(())
    }

    fn show(&self) -> Result<(), Error> {
        match select_output() {
            Medium::Browser => self.show_in_browser(),
            Medium::Jupyter => {
                self.evcxr_display();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Medium {
    Noop,
    Auto,
    Jupyter,
    Browser,
}

pub static OUTPUT: Medium = Medium::Auto;

fn select_output() -> Medium {
    use std::env;
    match OUTPUT {
        Medium::Auto if env::var("EVCXR_IS_RUNTIME").is_ok() => Medium::Jupyter,
        Medium::Auto => Medium::Browser,
        ref m => m.clone(),
    }
}

const CONTENT_EMBED_HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
</head>
<body>

<div id="vis">
{{ content }}
</div>
</body>
</html>
"#;

/// Display content as text
///
/// ```rust
/// use showata::*;
///
/// let content = ".....";
/// show_text_in_jupyter("text/plain", content);
/// ```
/// TODO replace by evcxr_runtime ?
pub fn show_text_in_jupyter<M: AsRef<str>, S: AsRef<str>>(mime_type: M, text: S) {
    println!(
        "EVCXR_BEGIN_CONTENT {}\n{}\nEVCXR_END_CONTENT",
        mime_type.as_ref(),
        text.as_ref()
    );
}

/// Display bytes as base64 encoded
///
/// ```rust
/// use showata::*;
///
/// let buffer: Vec<u8> = vec![];
/// show_bytes_in_jupyter("image/png", &buffer);
/// ```
// TODO replace by evcxr_runtime ?
pub fn show_bytes_in_jupyter<S: AsRef<str>>(mime_type: S, buffer: &[u8]) {
    show_text_in_jupyter(mime_type, base64::encode(buffer))
}

/// Display the content of a local file.
///
/// ```rust
/// use showata::*;
/// use mime;
///
/// show_file_in_jupyter("local-img.png", "image/png", false);
/// show_file_in_jupyter("local-img.png", mime::IMAGE_PNG, false);
///
/// show_file_in_jupyter("hello.html", "text/html", true);
/// show_file_in_jupyter("hello.svg", mime::IMAGE_SVG, true);
/// ```
pub fn show_file_in_jupyter<P: AsRef<Path>, S: AsRef<str>>(
    path: P,
    mime_type: S,
    as_text: bool,
) -> Result<(), std::io::Error> {
    let buffer = std::fs::read(path)?;
    if as_text {
        let text = String::from_utf8_lossy(&buffer);
        show_text_in_jupyter(mime_type, &text);
    } else {
        show_bytes_in_jupyter(mime_type, &buffer);
    }
    Ok(())
}

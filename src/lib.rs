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
mod display_iterator;

#[cfg(feature = "display_nalgebra")]
mod display_nalgebra;

#[cfg(feature = "display_ndarray")]
mod display_ndarray;

#[cfg(feature = "display_vega")]
mod display_vega;
mod vegalite;

use std::path::Path;
use failure::Error;

pub struct ContentInfo {
    pub mime_type: String,
    pub content: String,
}

pub trait Displayable {
    fn to_content_info(&self) -> Result<ContentInfo, Error>;

    fn evcxr_display(&self) {
        let ci = self.to_content_info().expect("to be convertible into ContentInfo");
        jupyter_display_text(ci.mime_type, ci.content);
    }

    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = CONTENT_EMBED_HTML_TEMPLATE.replace("{{ content }}", &dod.content);
        Ok(content.into())
    }

    fn browser_display(&self) -> Result<(), Error> {
        let mut dir = std::env::temp_dir();
        dir.push(env!("CARGO_PKG_NAME"));
        //TODO security check that the user can read/write only his own files
        std::fs::create_dir_all(&dir)?;
        //TODO generate random/timestamp file
        let path = dir.join("display.html");
        let html = self.to_html_page()?;
        std::fs::write(&path, html)?;
        opener::open(path.as_os_str())?;
        Ok(())
    }

    fn display(&self) -> Result<(), Error> {
        match select_output() {
            DisplayOutput::Noop => Ok(()),
            DisplayOutput::Browser => self.browser_display(),
            DisplayOutput::Jupyter => {
                self.evcxr_display();
                Ok(())
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DisplayOutput {
    Noop,
    Jupyter,
    Browser,
}

pub static OUTPUT: DisplayOutput = DisplayOutput::Browser;

fn select_output() -> &'static DisplayOutput {
    &OUTPUT //DisplayOutput::Browser
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
/// use evcxr_displayers::*;
///
/// let content = ".....";
/// display_text("text/plain", content);
/// ```
/// TODO replace by evcxr_runtime ?
pub fn jupyter_display_text<M: AsRef<str>, S: AsRef<str>>(mime_type: M, text: S) {
    println!(
        "EVCXR_BEGIN_CONTENT {}\n{}\nEVCXR_END_CONTENT",
        mime_type.as_ref(),
        text.as_ref()
    );
}

/// Display bytes as base64 encoded
///
/// ```rust
/// use evcxr_displayers::*;
///
/// let buffer: Vec<u8> = vec![];
/// display_bytes("image/png", &buffer);
/// ```
// TODO replace by evcxr_runtime ?
pub fn jupyter_display_bytes<S: AsRef<str>>(mime_type: S, buffer: &[u8]) {
    jupyter_display_text(mime_type, base64::encode(buffer))
}

/// Display the content of a local file.
///
/// ```rust
/// use evcxr_displayers::*;
/// use mime;
///
/// display_file("local-img.png", "image/png", false);
/// display_file("local-img.png", mime::IMAGE_PNG, false);
///
/// display_file("hello.html", "text/html", true);
/// display_file("hello.svg", mime::IMAGE_SVG, true);
/// ```
pub fn jupyter_display_file<P: AsRef<Path>, S: AsRef<str>>(
    path: P,
    mime_type: S,
    as_text: bool,
) -> Result<(), std::io::Error> {
    let buffer = std::fs::read(path)?;
    if as_text {
        let text = String::from_utf8_lossy(&buffer);
        jupyter_display_text(mime_type, &text);
    } else {
        jupyter_display_bytes(mime_type, &buffer);
    }
    Ok(())
}

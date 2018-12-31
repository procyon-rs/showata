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

#[cfg(feature="display_nalgebra")]
mod display_nalgebra;

use std::path::Path;

pub trait EvcxrDisplay {
    fn evcxr_display(&self);
}

/// Display content as text
///
/// ```rust
/// use evcxr_displayers::*;
///
/// let content = ".....";
/// display_text("text/plain", content);
/// ```
/// TODO replace by evcxr_runtime ?
pub fn display_text<M: AsRef<str>, S: AsRef<str>>(mime_type: M, text: S) {
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
pub fn display_bytes<S: AsRef<str>>(mime_type: S, buffer: &[u8]) {
    display_text(mime_type, base64::encode(buffer))
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
pub fn display_file<P: AsRef<Path>, S: AsRef<str>>(path: P, mime_type: S, as_text: bool) -> Result<(), std::io::Error> {
    let buffer = std::fs::read(path)?;
    if as_text {
        let text = String::from_utf8_lossy(&buffer);
        display_text(mime_type, &text);
    } else {
        display_bytes(mime_type, &buffer);
    }
    Ok(())
}

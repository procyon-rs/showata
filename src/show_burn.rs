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
use burn_tensor;
use core::fmt::{Debug, Write};

impl<B, const D: usize, K> Showable for burn_tensor::Tensor<B, D, K>
where
    B: burn_tensor::backend::Backend,
    K: burn_tensor::TensorKind<B> + burn_tensor::BasicOps<B>,
    K::Elem: Debug,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        self.to_data().to_content_info()
    }
}

impl<B, const D: usize, K> Showable for &burn_tensor::Tensor<B, D, K>
where
    B: burn_tensor::backend::Backend,
    K: burn_tensor::TensorKind<B> + burn_tensor::BasicOps<B>,
    K::Elem: Debug,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        (*self).to_content_info()
    }
}

const START: &str = "tensor([";
const END: &str = "])";

impl<P, const D: usize> Showable for burn_tensor::Data<P, D>
where
    P: Debug,
{
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        let dims = self.shape.dims;
        let data = &self.value;

        // if single-valued 1D tensor, just print the value
        if dims.as_ref() == &[1] {
            return Ok(ContentInfo {
                mime_type: "text/plain".into(),
                content: format!("tensor({:?})", data[0]),
            });
        }

        let col_size = match dims.len() {
            1 => dims[0],
            2 => dims[1],
            _ => unimplemented!("only supports 1D/2D tensors"),
        };
        let row_size = data.len() / col_size;

        let mut out = String::new();
        let padding = " ".repeat(START.len());

        write!(out, "{}", START)?;
        for (r, row) in data.as_slice().chunks(col_size).enumerate() {
            let (pre, post) = match r {
                0 => ("", ",\n"),
                _ if r < row_size - 1 => (padding.as_str(), ",\n"),
                _ => (padding.as_str(), END),
            };

            write!(out, "{}{:?}{}", pre, row, post)?;
        }

        Ok(ContentInfo {
            mime_type: "text/plain".into(),
            content: out,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Showable;

    #[test]
    fn test_no_crash_on_3x4() {
        use burn_tensor::Tensor;
        let m = Tensor::<_, 2>::zeros([3, 4]);
        m.show().unwrap();
    }

    #[test]
    fn test_no_crash_on_3x1() {
        use burn_tensor::Tensor;
        let m = Tensor::<_, 1>::zeros([3]);
        m.show().unwrap();
    }
}

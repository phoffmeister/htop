/*
 * MIT License
 *
 * Copyright (c) 2023 senees
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2023 senees
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! # Paper definitions

//TODO Add paper sizes from https://papersizes.io

use crate::errors::{err_invalid_paper_format, HtopError};

/// Length of the inch expressed in millimeters.
const MM_PER_INCH: f64 = 25.4;

/// Paper size definitions.
pub enum PaperSize {
  A0,
  A1,
  A2,
  A3,
  A4,
  A5,
  A6,
}

impl TryFrom<&String> for PaperSize {
  type Error = HtopError;
  /// Converts [PaperSize] from a reference to [String].
  fn try_from(value: &String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "A0" => Ok(Self::A0),
      "A1" => Ok(Self::A1),
      "A2" => Ok(Self::A2),
      "A3" => Ok(Self::A3),
      "A4" => Ok(Self::A4),
      "A5" => Ok(Self::A5),
      "A6" => Ok(Self::A6),
      other => Err(err_invalid_paper_format(other)),
    }
  }
}

/// Paper properties.
pub struct Paper {
  /// Paper width in inches.
  width: f64,
  /// Paper height in inches.
  height: f64,
}

impl Paper {
  /// Creates a new paper with specified size.
  pub fn new(paper_size: PaperSize) -> Self {
    // get paper size in millimeters
    let (width, height) = match paper_size {
      PaperSize::A0 => (841.0, 1189.0),
      PaperSize::A1 => (594.0, 841.0),
      PaperSize::A2 => (420.0, 594.0),
      PaperSize::A3 => (297.0, 420.0),
      PaperSize::A4 => (210.0, 297.0),
      PaperSize::A5 => (148.0, 210.0),
      PaperSize::A6 => (105.0, 148.0),
    };
    // create paper with sizes in inches
    Self {
      width: width / MM_PER_INCH,
      height: height / MM_PER_INCH,
    }
  }

  /// Returns paper width in inches.
  pub fn width(&self) -> f64 {
    self.width
  }

  /// Returns paper height in inches.
  pub fn height(&self) -> f64 {
    self.height
  }
}

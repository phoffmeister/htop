/*
 * MIT License
 *
 * Copyright (c) 2023 senees
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
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

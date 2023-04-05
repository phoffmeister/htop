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

//! # Errors

use std::fmt;
use std::path::Path;

/// Common result type.
pub type Result<T, E = HtopError> = std::result::Result<T, E>;

/// Common error definition.
#[derive(PartialEq, Eq)]
pub struct HtopError(String);

impl fmt::Display for HtopError {
  /// Implements [Display](fmt::Display) trait for [HtopError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::Debug for HtopError {
  /// Implements [Debug](fmt::Debug) trait for [HtopError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl HtopError {
  /// Creates a new [HtopError] with specified message.
  pub fn new(message: String) -> Self {
    Self(message)
  }
}

/// Creates invalid paper format error.
pub fn err_invalid_paper_format(format_name: &str) -> HtopError {
  HtopError::new(format!("invalid paper format '{}'", format_name))
}

/// Creates an error with failure reason message from headless chrome.
pub fn err_headless_chrome(reason: String) -> HtopError {
  HtopError::new(format!("headless chrome failed with reason: {}", reason))
}

/// Creates an error with file writing failure reason.
pub fn err_write_file(file_name: &str, reason: String) -> HtopError {
  HtopError::new(format!("writing file {} failed with reason: {}", file_name, reason))
}

/// Creates an error when canonicalizing a path fails.
pub fn err_canonicalize(path: &Path, reason: String) -> HtopError {
  HtopError::new(format!(
    "canonicalizing failed for path {} with reason: {}",
    path.to_string_lossy(),
    reason
  ))
}

/// Creates an error when retrieving file name fails.
pub fn err_file_name(path: &Path) -> HtopError {
  HtopError::new(format!(
    "retrieving file name for path {} failed",
    path.to_string_lossy()
  ))
}

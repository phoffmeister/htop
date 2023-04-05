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

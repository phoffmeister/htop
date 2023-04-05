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

//! # Utility functions

use crate::defs::{HTML_EXTENSION, PDF_EXTENSION};
use crate::errors::{err_canonicalize, err_file_name, Result};
use std::env;
use std::path::Path;

/// Converts file path into file URL string.
pub fn file_url(file_path: &Path) -> Result<String> {
  Ok(format!(
    "file://{}",
    file_path
      .canonicalize()
      .map_err(|e| err_canonicalize(file_path, e.to_string()))?
      .to_string_lossy()
  ))
}

/// Replaces the extension to `.pdf`.
pub fn replace_ext(path: &Path) -> String {
  path.with_extension(PDF_EXTENSION).to_string_lossy().to_string()
}

/// Replaces the extension to `.pdf` and returns the file name.
pub fn file_name(path: &Path) -> Result<String> {
  Ok(
    path
      .with_extension(PDF_EXTENSION)
      .file_name()
      .ok_or(err_file_name(path))?
      .to_string_lossy()
      .to_string(),
  )
}

/// Returns `true` when specified path has `HTML` file extension.
pub fn has_html_extension(path: &Path) -> bool {
  if let Some(extension) = path.extension() {
    extension == HTML_EXTENSION
  } else {
    false
  }
}

/// Initializes the logger.
pub fn init_logger(opt_log_level: Option<String>) {
  match env::var("RUST_LOG").unwrap_or("off".to_string()).as_str() {
    "error" | "warn" | "info" | "debug" | "trace" => {}
    _ => env::set_var("RUST_LOG", "off"),
  }
  if let Some(log_level) = opt_log_level {
    env::set_var("RUST_LOG", log_level);
  }
  env_logger::init();
}

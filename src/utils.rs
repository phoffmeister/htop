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

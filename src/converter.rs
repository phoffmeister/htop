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

//! # HTML to PDF converter

use crate::defs::{Files, HTOP_NAME};
use crate::errors::{err_headless_chrome, err_write_file, Result};
use crate::options::PdfPrintingOptions;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::fs;

/// Converts `HTML` input files into `PDF` output files.
pub fn html_to_pdf(files: Files, pdf_printing_options: PdfPrintingOptions) -> Result<()> {
  let verbose = pdf_printing_options.verbose;
  let no_crash_reports = pdf_printing_options.no_crash_reports;
  let arguments = if no_crash_reports {
    vec![OsStr::new("--disable-crash-reporter")]
  } else {
    vec![]
  };
  let options = LaunchOptionsBuilder::default()
    .args(arguments)
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  let browser = Browser::new(options).map_err(|e| err_headless_chrome(e.to_string()))?;
  let tab = browser.new_tab().map_err(|e| err_headless_chrome(e.to_string()))?;
  for (input_url, output_file_name) in &files {
    if verbose {
      println!("[{HTOP_NAME}] Printing file {}", input_url);
    }
    tab
      .navigate_to(input_url)
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    tab
      .wait_until_navigated()
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    let pdf = tab
      .print_to_pdf(Some(pdf_printing_options.into()))
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    fs::write(output_file_name, pdf).map_err(|e| err_write_file(output_file_name, e.to_string()))?;
    if verbose {
      println!("[{HTOP_NAME}] Printing completed: {}\n", output_file_name);
    }
  }
  Ok(())
}

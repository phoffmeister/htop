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

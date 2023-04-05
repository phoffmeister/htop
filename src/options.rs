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

//! # PDF printing options

use headless_chrome::types::PrintToPdfOptions;

#[derive(Copy, Clone)]
pub struct PdfPrintingOptions {
  /// Paper mode, `true` = landscape, `false` = portrait.
  pub landscape: bool,
  /// Flag indicating it background should be printed, `true` = print background.
  pub print_background: bool,
  /// Paper width in inches.
  pub paper_width: f64,
  /// Paper height in inches.
  pub paper_height: f64,
  /// Flag indicating if printing process should be more _talkative_.
  pub verbose: bool,
}

impl From<PdfPrintingOptions> for PrintToPdfOptions {
  fn from(value: PdfPrintingOptions) -> Self {
    Self {
      landscape: Some(value.landscape),
      display_header_footer: None,
      print_background: Some(value.print_background),
      scale: None,
      paper_width: Some(value.paper_width),
      paper_height: Some(value.paper_height),
      margin_top: None,
      margin_bottom: None,
      margin_left: None,
      margin_right: None,
      page_ranges: None,
      ignore_invalid_page_ranges: None,
      header_template: None,
      footer_template: None,
      prefer_css_page_size: None,
      transfer_mode: None,
    }
  }
}

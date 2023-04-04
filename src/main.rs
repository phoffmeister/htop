/*
 * MIT License
 *
 * Copyright (c) 2015-2023 senees
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

use clap::{arg, command};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::Browser;
use log::info;
use std::path::Path;
use std::{env, fs};

/// Paper properties.
struct Paper(f64, f64);

const PAPER_A4: Paper = Paper(8.3, 11.7);

/// Converts `HTML` input file into `PDF` output file.
fn html_to_pdf(input_url: &str) -> Vec<u8> {
  let browser = Browser::default().unwrap();
  let tab = browser.new_tab().unwrap();
  tab.navigate_to(input_url).unwrap();
  tab.wait_until_navigated().unwrap();
  // prepare PDF printing options
  let options = PrintToPdfOptions {
    landscape: Some(false),
    display_header_footer: Some(false),
    print_background: Some(true),
    scale: None,
    paper_width: Some(PAPER_A4.0),
    paper_height: Some(PAPER_A4.1),
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
  };
  tab.print_to_pdf(Some(options)).unwrap()
}

/// Main entrypoint of the application.
fn main() {
  env::set_var("RUST_LOG", "info");
  pretty_env_logger::init();

  let matches = command!()
    .name("htop")
    .arg(arg!(<INPUT_FILE>).help("Input HTML file name").required(true).index(1))
    .arg(arg!(<OUTPUT_FILE>).help("Output PDF file name").required(true).index(2))
    .get_matches();

  // prepare the input file URL
  let Some(input_file) = matches.get_one::<String>("INPUT_FILE") else {
    return;
  };
  let input_file_path = Path::new(input_file).canonicalize().unwrap();
  let input_file_url = format!("file://{}", input_file_path.to_string_lossy());

  // prepare the output file name
  let Some(output_file) = matches.get_one::<String>("OUTPUT_FILE") else {
    return;
  };
  let output_file_path = Path::new(output_file).canonicalize().unwrap();
  let output_file_name = output_file_path.to_string_lossy().to_string();

  info!(" Input file: {}", input_file_url);
  info!("Output file: {}", output_file_name);

  let pdf_content = html_to_pdf(&input_file_url);
  fs::write(output_file_name, pdf_content).unwrap();
}

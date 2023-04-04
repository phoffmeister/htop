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

use clap::{arg, command, Command};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::Browser;
use log::info;
use std::path::Path;
use std::{env, fs};

/// Paper properties.
struct Paper(f64, f64);

const MM_PER_INCH: f64 = 25.4;

const PAPER_A4: Paper = Paper(210.0 / MM_PER_INCH, 297.0 / MM_PER_INCH);

/// Converts `HTML` input file into `PDF` output file.
fn html_to_pdf(input_url: &str) -> Vec<u8> {
  info!("Opening browser");
  let browser = Browser::default().unwrap();
  info!("Opening new tab");
  let tab = browser.new_tab().unwrap();
  info!("Opening an input file in new tab");
  tab.navigate_to(input_url).unwrap();
  info!("Waiting until the input file is opened");
  tab.wait_until_navigated().unwrap();
  // prepare PDF printing options
  let pdf_options = PrintToPdfOptions {
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
  info!("Starting printing");
  let pdf_content = tab.print_to_pdf(Some(pdf_options)).unwrap();
  info!("Printing completed");
  pdf_content
}

/// Main entrypoint of the application.
fn main() {
  env::set_var("RUST_LOG", "info");
  env_logger::init();

  let matches = command!()
    .name("htop")
    .subcommand(
      Command::new("single")
        .about("Convert single HTML file to PDF")
        .display_order(1)
        .arg(arg!(<INPUT_FILE>).help("Input HTML file name").required(true).index(1))
        .arg(arg!(<OUTPUT_FILE>).help("Output PDF file name").required(true).index(2)),
    )
    .subcommand(
      Command::new("multiple")
        .about("Convert multiple HTML files to PDF")
        .display_order(2)
        .arg(arg!(<INPUT_DIR>).help("Input directory").required(true).index(1)),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("single", matches)) => {
      // prepare the input file URL
      let Some(input_file) = matches.get_one::<String>("INPUT_FILE") else {
        return;
      };
      let input_file_path = Path::new(input_file).canonicalize().unwrap();
      let input_file_url = format!("file://{}", input_file_path.to_string_lossy());

      // prepare the output file name
      let Some(output_file_name) = matches.get_one::<String>("OUTPUT_FILE") else {
        return;
      };

      info!("Input file: {}", input_file_url);
      info!("Output file: {}", output_file_name);

      let pdf_content = html_to_pdf(&input_file_url);
      info!("Writing output file");
      fs::write(output_file_name, pdf_content).unwrap();
      info!("Conversion completed");
    }
    _ => {}
  }
}

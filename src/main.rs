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

use clap::{arg, command, ArgAction, ArgMatches};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::Browser;
use log::info;
use std::path::Path;
use std::{env, fs};

/// Paper properties, currently just width and height in inches.
struct Paper(f64, f64);

/// Length of the inch expressed in millimeters.
const MM_PER_INCH: f64 = 25.4;

/// `A4` paper format.
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

const HELP_LANDSCAPE: &str = r#"Sets the paper orientation to landscape. In landscape mode,
the longest paper edge is positioned in horizontal direction"#;

const HELP_BACKGROUND: &str = r#"Prints also the backround of the page"#;

const HELP_SINGLE: &str = r#"Convert single HTML file to PDF"#;

const HELP_MULTIPLE: &str = r#"Convert multiple HTML files to PDF files"#;

const HELP_IN_FILE: &str = r#"Input HTML file"#;

const HELP_OUT_FILE: &str = r#"Output PFD file"#;

const HELP_IN_DIR: &str = r#"Input directory"#;

const HELP_OUT_DIR: &str = r#"Output directory"#;

/// Returns command-line arguments matches.
#[rustfmt::skip]
fn get_matches() -> ArgMatches {
  command!()
    .name("htop")
    .arg(arg!(-b --background).help(HELP_BACKGROUND).action(ArgAction::SetTrue).display_order(1))
    .arg(arg!(-l --landscape).help(HELP_LANDSCAPE).action(ArgAction::SetTrue).display_order(2))
    .subcommand(command!().name("single").about(HELP_SINGLE).display_order(1)
      .arg(arg!(<INPUT_FILE>).help(HELP_IN_FILE).required(true).index(1))
      .arg(arg!(<OUTPUT_FILE>).help(HELP_OUT_FILE).required(false).index(2)))
    .subcommand(command!().name("multiple").about(HELP_MULTIPLE).display_order(2)
      .arg(arg!(<INPUT_DIR>).help(HELP_IN_DIR).required(true).index(1))
      .arg(arg!(<OUTPUT_DIR>).help(HELP_OUT_DIR).required(false).index(2)))
    .get_matches()
}

/// Main entrypoint of the application.
fn main() {
  env::set_var("RUST_LOG", "info");
  env_logger::init();

  let matches = get_matches();

  match matches.subcommand() {
    Some(("single", m)) => {
      // input file name is required
      let input_file = m.get_one::<String>("INPUT_FILE").unwrap();
      let input_file_path = Path::new(input_file).canonicalize().unwrap();
      let input_file_url = format!("file://{}", input_file_path.to_string_lossy());

      // output file name is optional
      let output_file_path = if let Some(output_file_name) = m.get_one::<String>("OUTPUT_FILE") {
        Path::new(output_file_name).to_owned()
      } else {
        let mut path = input_file_path.clone();
        path.set_extension("pdf");
        path
      };

      info!("Input file: {}", input_file_path.to_string_lossy());
      info!("Input file URL: {}", input_file_url);
      info!("Output file: {}", output_file_path.to_string_lossy());

      let pdf_content = html_to_pdf(&input_file_url);
      info!("Writing output file");
      fs::write(output_file_path, pdf_content).unwrap();
      info!("Conversion completed");
    }
    Some(("multiple", _m)) => {}
    _ => {}
  }
}

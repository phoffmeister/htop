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

use clap::{arg, command, ArgAction, ArgMatches};
use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::Browser;
use log::info;
use std::path::Path;
use std::{env, fs};

const HTOP_VERSION: &str = env!("CARGO_PKG_VERSION");
const HTOP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Paper properties, currently just width and height in inches.
struct Paper(f64, f64);

/// Length of the inch expressed in millimeters.
const MM_PER_INCH: f64 = 25.4;

/// `A4` paper format.
const PAPER_A4: Paper = Paper(210.0 / MM_PER_INCH, 297.0 / MM_PER_INCH);

/// Converts `HTML` input files into `PDF` output files.
fn html_to_pdf(files: Vec<(String, String)>) {
  let browser = Browser::default().unwrap();
  let tab = browser.new_tab().unwrap();
  for (input_url, output_file_name) in &files {
    info!("Printing file: {}", input_url);
    tab.navigate_to(input_url).unwrap();
    tab.wait_until_navigated().unwrap();
    let pdf_options = Some(PrintToPdfOptions {
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
    });
    let pdf = tab.print_to_pdf(pdf_options).unwrap();
    fs::write(output_file_name, pdf).unwrap();
    info!("Printing completed: {}", output_file_name);
  }
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

const PDF_EXTENSION: &str = "pdf";

/// Returns command-line arguments matches.
#[rustfmt::skip]
fn get_matches() -> ArgMatches {
  command!()
    .name("htop")
    .arg(arg!(-b --background).help(HELP_BACKGROUND).action(ArgAction::SetTrue).display_order(1))
    .arg(arg!(-l --landscape).help(HELP_LANDSCAPE).action(ArgAction::SetTrue).display_order(2))
    .subcommand(command!().name("single").about(HELP_SINGLE).display_order(1)
      .arg(arg!(<INPUT_FILE>).help(HELP_IN_FILE).required(true).index(1))
      .arg(arg!([OUTPUT_FILE]).help(HELP_OUT_FILE).required(false).index(2)))
    .subcommand(command!().name("multiple").about(HELP_MULTIPLE).display_order(2)
      .arg(arg!(<INPUT_DIR>).help(HELP_IN_DIR).required(true).index(1))
      .arg(arg!([OUTPUT_DIR]).help(HELP_OUT_DIR).required(false).index(2)))
    .get_matches()
}

fn file_url(file_path: &Path) -> String {
  format!("file://{}", file_path.canonicalize().unwrap().to_string_lossy())
}

fn replace_ext(file_path: &Path) -> String {
  let mut path = file_path.canonicalize().unwrap().to_path_buf();
  path.set_extension(PDF_EXTENSION);
  path.to_string_lossy().to_string()
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
      let input_file_path = Path::new(input_file);
      let input_file_url = file_url(input_file_path);
      // output file name is optional
      let output_file_name = if let Some(output_file) = m.get_one::<String>("OUTPUT_FILE") {
        output_file.to_owned()
      } else {
        replace_ext(input_file_path)
      };
      // convert files
      html_to_pdf(vec![(input_file_url, output_file_name)]);
    }
    Some(("multiple", m)) => {
      let mut files = vec![];
      // input directory name is required
      let input_dir = m.get_one::<String>("INPUT_DIR").unwrap();
      // output directory is optional
      if let Some(output_dir) = m.get_one::<String>("OUTPUT_DIR") {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() {
            let input_file_url = file_url(&entry);
            let output_file_path = Path::new(output_dir).join(entry.file_name().unwrap());
            let output_file_name = replace_ext(output_file_path.as_path());
            files.push((input_file_url, output_file_name));
          }
        }
      } else {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() {
            let input_file_url = file_url(&entry);
            let output_file_name = replace_ext(entry.as_path());
            files.push((input_file_url, output_file_name));
          }
        }
      }
      // convert files
      html_to_pdf(files);
    }
    _ => {
      println!("htop {HTOP_VERSION}");
      println!("{HTOP_DESCRIPTION}\n");
      println!("htop: missing subcommand");
      println!("Try 'htop --help' for more information.\n");
    }
  }
}

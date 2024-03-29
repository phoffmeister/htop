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

use crate::converter::html_to_pdf;
use crate::defs::*;
use crate::errors::Result;
use crate::options::PdfPrintingOptions;
use crate::paper::Paper;
use crate::utils::*;
use clap::{arg, command, ArgAction, ArgMatches};
use std::path::Path;
use std::{env, fs};

mod converter;
mod defs;
mod errors;
mod options;
mod paper;
mod utils;

/// Returns command-line arguments matches.
#[rustfmt::skip]
fn get_matches() -> ArgMatches {
  command!()
    .name(HTOP_NAME)
    .arg(arg!(-b --background).help(HELP_BACKGROUND).action(ArgAction::SetTrue).display_order(1))
    .arg(arg!(-l --landscape).help(HELP_LANDSCAPE).action(ArgAction::SetTrue).display_order(2))
    .arg(arg!(--paper <FORMAT>).help(HELP_PAPER).action(ArgAction::Set).default_value("A4").default_missing_value("A4").display_order(3))
    .arg(arg!(-v --verbose).help(HELP_VERBOSE).action(ArgAction::SetTrue).display_order(4))
    .arg(arg!(--"log-level" <LEVEL>).help(HELP_LOG_LEVEL).action(ArgAction::Set).default_missing_value("off").display_order(5))
    .arg(arg!(--"no-crash-reports").help(HELP_NO_CRASH_REPORTS).action(ArgAction::SetTrue).display_order(6))
    .subcommand(command!().name(SUBCOMMAND_SINGLE).about(HELP_SINGLE).display_order(1)
      .arg(arg!(<INPUT_FILE>).help(HELP_IN_FILE).required(true).index(1))
      .arg(arg!([OUTPUT_FILE]).help(HELP_OUT_FILE).required(false).index(2)))
    .subcommand(command!().name(SUBCOMMAND_MULTIPLE).about(HELP_MULTIPLE).display_order(2)
      .arg(arg!(<INPUT_DIR>).help(HELP_IN_DIR).required(true).index(1))
      .arg(arg!([OUTPUT_DIR]).help(HELP_OUT_DIR).required(false).index(2)))
    .get_matches()
}

/// Main entrypoint of the application.
fn main() -> Result<()> {
  // get command-line argument matches
  let matches = get_matches();

  // initialize the logger
  init_logger(matches.get_one::<String>("log-level").cloned());

  // parse options
  let landscape = matches.get_flag("landscape");
  let print_background = matches.get_flag("background");
  let verbose = matches.get_flag("verbose");
  let paper_format = matches.get_one::<String>("paper").unwrap();
  let paper = Paper::new(paper_format.try_into()?);
  let no_crash_reports = matches.get_flag("no-crash-reports");
  let pdf_printing_options = PdfPrintingOptions {
    landscape,
    print_background,
    paper_width: paper.width(),
    paper_height: paper.height(),
    verbose,
    no_crash_reports,
  };

  // parse subcommands
  match matches.subcommand() {
    Some((SUBCOMMAND_SINGLE, m)) => {
      // input file name is required
      let input_file = m.get_one::<String>("INPUT_FILE").unwrap();
      let input_file_path = Path::new(input_file);
      let input_file_url = file_url(input_file_path)?;
      // output file name is optional
      let output_file_name = if let Some(output_file) = m.get_one::<String>("OUTPUT_FILE") {
        output_file.to_owned()
      } else {
        replace_ext(input_file_path)
      };
      // convert files
      html_to_pdf(vec![(input_file_url, output_file_name)], pdf_printing_options)?;
    }
    Some((SUBCOMMAND_MULTIPLE, m)) => {
      let mut files: Files = vec![];
      // input directory name is required
      let input_dir = m.get_one::<String>("INPUT_DIR").unwrap();
      // output directory is optional
      if let Some(output_dir) = m.get_one::<String>("OUTPUT_DIR") {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url(&entry)?;
            let output_file_path = Path::new(output_dir).join(file_name(entry.as_path())?);
            let output_file_name = output_file_path.to_string_lossy().to_string();
            files.push((input_file_url, output_file_name));
          }
        }
      } else {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url(&entry)?;
            let output_file_name = replace_ext(entry.as_path());
            files.push((input_file_url, output_file_name));
          }
        }
      }
      // convert files
      html_to_pdf(files, pdf_printing_options)?;
    }
    _ => {
      println!("{HTOP_NAME} {HTOP_VERSION}\n{HTOP_DESCRIPTION}\n");
      println!("{HTOP_NAME}: missing subcommand");
      println!("Try '{HTOP_NAME} --help' for more information.");
    }
  }
  Ok(())
}

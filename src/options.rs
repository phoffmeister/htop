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

//! # PDF printing options

use headless_chrome::types::PrintToPdfOptions;

/// PDF printing options.
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
  /// Flag indicating if crash reporter should be disabled.
  pub no_crash_reports: bool,
}

impl From<PdfPrintingOptions> for PrintToPdfOptions {
  /// Converts [PdfPrintingOptions] into [PrintToPdfOptions].
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

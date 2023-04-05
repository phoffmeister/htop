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

//! # Common definitions

/// Type alias for a collection of file tuples.
pub type Files = Vec<(String, String)>;

pub const HTOP_NAME: &str = env!("CARGO_PKG_NAME");

pub const HTOP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HTOP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const SUBCOMMAND_SINGLE: &str = "single";

pub const SUBCOMMAND_MULTIPLE: &str = "multiple";

pub const HELP_BACKGROUND: &str = r#"Prints also the backround of the page"#;

pub const HELP_LANDSCAPE: &str = r#"Sets the paper orientation to landscape. In landscape mode,
the longest paper edge is positioned in horizontal direction"#;

pub const HELP_PAPER: &str = r#"Paper format like A4 (default), A3, A2 and more"#;

pub const HELP_VERBOSE: &str = r#"Display printing process details"#;

pub const HELP_LOG_LEVEL: &str = r#"Logging level, allowed values are
error, warn, info, debug, trace, off (default"#;

pub const HELP_SINGLE: &str = r#"Convert single HTML file to PDF"#;

pub const HELP_MULTIPLE: &str = r#"Convert multiple HTML files to PDF files"#;

pub const HELP_IN_FILE: &str = r#"Input HTML file"#;

pub const HELP_OUT_FILE: &str = r#"Output PFD file"#;

pub const HELP_IN_DIR: &str = r#"Input directory"#;

pub const HELP_OUT_DIR: &str = r#"Output directory"#;

// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use clap::{arg, command, crate_authors, value_parser, Command};

/// Parses command-line arguments using the `clap` library.
///
/// # Returns
///
/// Returns an instance of `ArgMatches` which contains the parsed arguments.
///
pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        //.arg(arg!(--init ... "Init config.yaml"))
        .arg(
            arg!(-p --percentage <percentage> "Set brightness by percentage.")
                .required(false)
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(change: -c --change <change> "Change brightness by percentage.")
                .required(false)
                .allow_hyphen_values(true)
                .value_parser(value_parser!(i32)),
        )
        .arg(arg!(get: --get "Get current brightness.").required(false))
}

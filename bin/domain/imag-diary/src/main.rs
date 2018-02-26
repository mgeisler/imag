//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;
extern crate clap;
extern crate chrono;
extern crate toml;
extern crate toml_query;

extern crate libimagdiary;
extern crate libimagentryedit;
extern crate libimagerror;
extern crate libimaginteraction;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagtimeui;
extern crate libimagutil;

use std::io::Write;
use std::process::exit;

use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagrt::runtime::Runtime;

mod create;
mod delete;
mod edit;
mod list;
mod ui;
mod util;
mod view;

use create::create;
use delete::delete;
use edit::edit;
use list::list;
use ui::build_ui;
use view::view;

fn main() {
    let version = make_imag_version!();
    let name = "imag-diary";
    let about = "Personal Diary/Diaries";
    let ui = build_ui(Runtime::get_default_cli_builder(name, &version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            let _ = writeln!(rt.stdout(), "Could not set up Runtime").to_exit_code().unwrap_or_exit();
            let _ = writeln!(rt.stdout(), "{:?}", rt.err().unwrap()).to_exit_code().unwrap_or_exit();
            exit(1);
        }
    };

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit" => edit(&rt),
                "list" => list(&rt),
                "view" => view(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            }
        });
}


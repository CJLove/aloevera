// Copyright 2020 Revcore Technologies Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::str::FromStr;

use clap::ArgMatches;

use super::command::{self, AsmArgs, AsmSelectArgs};
use crate::cmd::common::{self, GlobalArgs};
use crate::util;
use crate::{Error, ErrorKind};

use vera::AsmFormat;

pub fn parse_asm_args(g_args: &GlobalArgs, args: &ArgMatches) -> Result<AsmArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let out_dir = common::parse_required(args, "out_dir")?;
	let asm_format = common::parse_required(args, "format")?;
	let sd_image = match args.value_of("sd_image") {
		Some(s) => Some(s.into()),
		None => None,
	};
	let conflate_tilemaps = args.is_present("conflate_tilemaps");
	Ok(AsmArgs {
		out_dir: out_dir.into(),
		format: AsmFormat::from_str(asm_format)?,
		sd_image,
		conflate_tilemaps,
	})
}

pub fn parse_asm_select_args(
	asm_args: &AsmArgs,
	args: &ArgMatches,
) -> Result<AsmSelectArgs, Error> {
	let asset_id = common::parse_required(args, "asset_id")?;
	let out_file = common::parse_required(args, "out_file")?;

	let mut bin_address = common::parse_required(args, "bin_address")?
		.to_owned()
		.to_uppercase();
	if bin_address.starts_with("0X") {
		bin_address = bin_address.split_off(2);
	};
	let bin_address_vec = util::hex::from_hex(bin_address)?;
	if bin_address_vec.len() != 2 {
		let msg = format!(".bin start address must be 16 bytes (4 hex digits)");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let mut bin_address = [0, 0];
	for i in 0..2 {
		bin_address[i] = bin_address_vec[i];
	}

	Ok(AsmSelectArgs {
		asset_id: asset_id.into(),
		out_file: format!(
			"{}{}{}",
			asm_args.out_dir,
			std::path::MAIN_SEPARATOR,
			out_file
		),
		bin_address,
	})
}

pub fn execute_asm_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	let a = arg_parse!(parse_asm_args(g_args, args));
	match args.subcommand() {
		("all", Some(_)) => command::asm_all(g_args, a),
		("select", Some(args)) => {
			let s = parse_asm_select_args(&a, &args)?;
			command::asm_select(g_args, a, &s)
		}
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera asm --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}

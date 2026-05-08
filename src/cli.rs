//! CLI Handling

use clap::{ArgAction, Parser};

/// Commandline arguments for sunshine
#[derive(Clone, Parser)]
pub(crate) struct Args {
	/// Location string to calculate sunrise and sunset for.
	/// Format as
	/// "@lat long", e.g. "@45.815 15.9819" or
	/// "#location", e.g. "#New York" or
	/// "." or empty for location based on geoip.
	pub(crate) location: Option<String>,

	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,
}

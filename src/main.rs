
//use clap::{Arg, ArgAction, ArgMatches, Command};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "mike", about = "GNU Make in Rust", version = "1.0.1")]
#[command(override_usage = "mike [options] [target] ...")]
#[repr(C)]
struct Args {
	/// Unconditionally make all targets
	#[arg(short = 'B', long)]
	always_make: bool,
	
	/// Change to DIRECTORY before doing anything
	#[arg(short = 'C', long)]
	directory: Option<String>,
	
	/// Print lots of debugging information
	#[arg(short)]
	debug_all: bool,
	
	/// Print various types of debugging information
	#[arg(long, value_name="FLAGS")]
	debug: Option<String>,
	
	/// Environment variables override makefiles
	#[arg(short, long)]
	environment_overrides: bool,
	
	/// Evaluate STRING as a makefile statement
	#[arg(short = 'E', long, value_name="STRING")]
	eval: Option<String>,
	
	/// Read FILE as a makefile
	#[arg(short, long, visible_alias="makefile")]
	file: Option<String>,
	
	/// Ignore errors from recipes
	#[arg(short, long)]
	ignore_errors: bool,
	
	/// Search DIRECTORY for included makefiles
	#[arg(short = 'I', long)]
	include_dir: Option<String>,
	
	/// Allow N jobs at once; infinite jobs with no arg
	#[arg(short, long, value_name="N")]
	jobs: Option<u8>,
	
	/// Keep going when some targets can't be made
	#[arg(short, long)]
	keep_going: bool,
	
	/// Don't start multiple jobs unless load is below N
	#[arg(short, long, value_name="N", visible_alias="max-load")]
	load_avegage: Option<f32>,
	
	/// Use the latest mtime between symlinks and target
	#[arg(short = 'L', long)]
	check_symlinks_times: bool,
	
	/// Don't actually run any recipe; just print them
	#[arg(short = 'n', long, visible_aliases=&["dry-run", "recon"])]
	just_print : bool,
	
	/// Consider FILE to be very old and don't remake it
	#[arg(short, long, value_name="FILE", visible_alias="assume-old")]
	old_file: Option<String>,
	
	/// Synchronize output of parallel jobs by TYPE
	#[arg(short='O', long, value_name="TYPE")]
	output_sync: Option<String>,

	/// Print make's internal database
	#[arg(short, long)]
	print_data_base: bool,

	/// Run no recipe; exit status says if up to date
	#[arg(short, long)]
	question: bool,
	
	/// Disable the built-in implicit rules
	#[arg(short='r', long)]
	no_builtin_rules: bool,
	
	/// Disable the built-in variable settings
	#[arg(short='R', long)]
	no_builtin_variables: bool,
	
	/// Don't echo recipes
	#[arg(short, long, visible_alias="quiet")]
	silent: bool,
	
	/// Echo recipes (disable --silent mode)
	#[arg(long)]
	no_silent: bool,

	/// Turns off -k
	#[arg(short='S', long, visible_alias="stop")]
	no_keep_going: bool,
	
	/// Touch targets instead of remaking them
	#[arg(short, long)]
	touch: bool,
	
	/// Print tracing information
	#[arg(long)]
	trace: bool,
	
//	/// Print the version number of mike and exit
//	#[arg(short, long)]
//	version: bool,
	
	/// Print the current directory
	#[arg(short='w', long)]
	print_directory: bool,
	
	/// Turn off -w, even if it was turned on implicitly
	#[arg(long)]
	no_print_directory: bool,
	
	/// Consider FILE to be infinitely new
	#[arg(short='W', long, value_name="FILE", visible_aliases=&["new-file", "assume-new"])]
	what_if: Option<String>,

	/// Warn when an undefined variable is referenced}
	#[arg(long)]
	warn_undefined_variables: bool
}

fn main() {
	let args = Args::parse();
	return libmake(args);
//	println!("{:?}", args);
//	println!("{:#?}", args);
}


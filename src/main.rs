use std::{env, process::ExitCode};

// mod lib;
use fibonacci as lib;

fn main() -> ExitCode {
  let flags = &mut lib::Flags {
    is_progression: false,
    is_strict: false,
    print_index: false,
  };

  // Get non-option arguments
  let args = {
    // Import commandline arguments
    let raw_args: Vec<String> = env::args().collect();

    // Get options
    match lib::get_opts(raw_args, flags) {
      Ok(args) => args,
      Err(exit_code) => return exit_code,
    }
  };

  lib::run(args, flags)
}

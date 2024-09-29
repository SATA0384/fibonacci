pub(crate) fn print_usage() {
  println!("Usage: fibonacci [--options] <num1> [<num2> ...]");
  println!("       Arguments in [] are optional.");
}

pub(crate) fn print_help() {
  print!("fibonacci - v");
  print_version();
  println!("Description: Calculate fibonacci numbers.");
  println!();
  print_usage();
  println!();
  println!("Options:");
  println!("  --print-index  | -i : Print index.");
  println!("  --progression  | -p : Print fibonacci progression up to given number.");
  println!("  --version      | -v : Print version and exit.");
  println!("  --help  |  -h  | -? : Print this messages and exit.");
}

pub(crate) fn print_version() {
  println!("{}", env!("CARGO_PKG_VERSION"));
}

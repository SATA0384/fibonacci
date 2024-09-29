use std::{env, process::exit};

mod fibonacci_docs;
use fibonacci_docs::*;

struct Flags {
  is_progression: bool,
  print_index: bool,
}

fn main() {
  let mut flags = Flags {
    is_progression: false,
    print_index: false,
  };

  // Import commandline arguments
  let args: Vec<String> = env::args().collect();

  // Get options
  let args = get_opts(args, &mut flags);

  // If no argument -> Print usage and quit
  if args.len() == 0 {
    print_usage();
    return;
  }

  // For each arguments
  'EachArg: for idx in args {
    // Numericalize
    let idx = match idx.trim().parse() {
      Ok(num) => num, // if numeric, shadowing "idx" as usize
      Err(_) => {
        println!("NaN");
        continue;
      } // if not numeric, print 'NaN' and skip
    };

    let mut fibo: [u128; 3] = [0, 1, 0];

    let print_fibo = |fibo: [u128; 3], i| {
      if flags.is_progression {
        if flags.print_index {
          print!("{i}: ");
        }
        println!("{}", fibo[i % 3]);
      }
    };

    // Calculate fibonacci
    for i in 0..=1 {
      print_fibo(fibo, i);
    }

    let mut is_overflowed;
    for i in 2..=idx {
      (fibo[i % 3], is_overflowed) = fibo[(i + 1) % 3].overflowing_add(fibo[(i + 2) % 3]);

      // Convert u128->f64 to treat lerge number (might happen error(誤差))
      if is_overflowed {
        let mut fibo = [fibo[0] as f64, fibo[1] as f64, fibo[2] as f64];
        let print_fibo = |fibo: [f64; 3], j| {
          if flags.is_progression {
            if flags.print_index {
              print!("{j}: ");
            }
            println!("{:E}", fibo[j % 3]);
          }
        };

        for j in i..=idx {
          fibo[j % 3] = fibo[(j + 1) % 3] + fibo[(j + 2) % 3];
          print_fibo(fibo, j);
        }

        // Print in exponential format
        if !flags.is_progression {
          if flags.print_index {
            print!("{idx}: ");
          }
          println!("{:E}", fibo[idx % 3]);
        }
        continue 'EachArg;
      }

      print_fibo(fibo, i);
    }

    // Print
    if !flags.is_progression {
      if flags.print_index {
        print!("{idx}: ");
      }
      println!("{}", fibo[idx % 3]);
    }
  }
}

fn get_opts(args: Vec<String>, flags: &mut Flags) -> Vec<String> {
  let mut args_noopts = Vec::new();

  for arg in args.into_iter().skip(1) {
    if arg.len() == 0 {
      continue;
    }

    if arg.chars().nth(0).unwrap() == '-' {
      // If argument is an --option | -o
      match arg.as_str() {
        "--progression" | "-p" => flags.is_progression = true,
        "--print-index" | "-i" => flags.print_index = true,
        "--help" | "-h" | "-?" => {
          print_help();
          exit(0);
        }
        "--version" | "-v" => {
          print_version();
          exit(0);
        }
        _ => eprintln!("Invalid Option: {arg}"),
      }
    } else {
      // If argument is not an option
      args_noopts.push(arg);
    }
  }
  args_noopts
}

use std::process::ExitCode;

pub mod docs;

pub struct Flags {
  pub is_progression: bool,
  pub is_strict: bool,
  pub print_index: bool,
}

macro_rules! parse_opt {
  ("indexing", $flags: expr) => {
    $flags.print_index = true
  };
  ("progression", $flags: expr) => {
    $flags.is_progression = true
  };
  ("strict", $flags: expr) => {
    $flags.is_strict = true
  };
  ("print-help") => {{
    docs::print_help();
    return Err(ExitCode::SUCCESS);
  }};
  ("print-version") => {{
    docs::print_version();
    return Err(ExitCode::SUCCESS);
  }};
  ("invalid", $flags: expr, $opt: expr) => {{
    eprintln!("Invalid Option: {}", $opt);
    if $flags.is_strict {
      return Err(ExitCode::from(1));
    }
  }};
}

pub fn get_opts(args: Vec<String>, flags: &mut Flags) -> Result<Vec<String>, ExitCode> {
  let mut args_noopts = Vec::new();

  for arg in args.into_iter().skip(1) {
    if arg.len() == 0 {
      continue;
    }

    if arg.chars().nth(0).unwrap() == '-' {
      // If argument is an --option | -o
      if 1 == arg.len() {
        // Ignore just '-'
        continue;
      }

      if arg.chars().nth(1).unwrap() == '-' {
        // Long option
        match arg.as_str() {
          "--progression" => parse_opt!("progression", flags),
          "--print-index" => parse_opt!("indexing", flags),
          "--strict" => parse_opt!("strict", flags),
          "--help" => parse_opt!("print-help"),
          "--version" => parse_opt!("print-version"),
          _ => parse_opt!("invalid", flags, arg),
        }
      } else {
        let short_opts = arg.chars().skip(1);
        // Short option
        for c in short_opts {
          match c {
            'i' => parse_opt!("indexing", flags),
            'p' => parse_opt!("progression", flags),
            's' => parse_opt!("strict", flags),
            'h' | '?' => parse_opt!("print-help"),
            'v' => parse_opt!("print-version"),
            _ => parse_opt!("invalid", flags, c),
          }
        }
      }
    } else {
      // If argument is not an option
      args_noopts.push(arg);
    }
  }
  Ok(args_noopts)
}

pub fn run(args: Vec<String>, flags: &mut Flags) -> ExitCode {
  // If no argument -> Print usage and quit
  if args.len() == 0 {
    docs::print_usage();
    return ExitCode::SUCCESS;
  }

  // For each arguments
  'EachArg: for idx in args {
    // Numericalize
    let idx = match idx.trim().parse() {
      Ok(num) => num, // if numeric, shadowing "idx" as usize
      Err(_) => {
        // if not numeric, print 'NaN' and skip
        if flags.print_index {
          print!("{idx}: ");
        }
        println!("NaN");

        if flags.is_strict {
          return ExitCode::FAILURE;
        }
        continue;
      }
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

    let print_fibo_exp = |j| {
      if flags.print_index {
        print!("{j}: ");
      }

      let j = j as i32;
      let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
      let fibo = (phi.powi(j) - (-phi).powi(-j)) / 5.0_f64.sqrt();
      println!("{:E}", fibo);
    };

    if 186 < idx && !flags.is_progression {
      // Print in exponential format
      print_fibo_exp(idx);
      return ExitCode::SUCCESS;
    }

    // Calculate fibonacci
    for i in 0..=1 {
      print_fibo(fibo, i);
    }

    let mut is_overflowed;
    for i in 2..=idx {
      (fibo[i % 3], is_overflowed) = fibo[(i + 1) % 3].overflowing_add(fibo[(i + 2) % 3]);

      // Convert u128->f64 to treat lerge number (might happen error(誤差))
      if is_overflowed {
        for j in i..=idx {
          print_fibo_exp(j);
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

  return ExitCode::SUCCESS;
}

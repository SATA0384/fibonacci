use std::env;

fn print_usage() {
  println!("Usage: fibonacci [num1] [num2] ...");
}

fn main() {
  // Import commandline arguments
  let args: Vec<String> = env::args().collect();

  // If no argument -> Print usage and quit
  if args.len() == 1 {
    print_usage();
    return;
  }

  // For each arguments
  'EachArg: for idx in &args[1..] {
    // Numericalize
    let idx = match idx.trim().parse() {
      Ok(num) => num, // if numeric, shadowing "idx" as usize
      Err(_) => {
        println!("NaN");
        continue;
      } // if not numeric, print 'NaN' and skip
    };

    // Calculate fibonacci
    let mut fibo: [u128; 3] = [0, 1, 1];
    let mut is_overflowed;
    for i in 3..=idx {
      (fibo[i % 3], is_overflowed) = fibo[(i + 1) % 3].overflowing_add(fibo[(i + 2) % 3]);

      // Convert u128->f64 to treat lerge number (might happen error(誤差))
      if is_overflowed {
        let mut fibo = [fibo[0] as f64, fibo[1] as f64, fibo[2] as f64];
        for j in i..=idx {
          fibo[j % 3] = fibo[(j + 1) % 3] + fibo[(j + 2) % 3];
        }

        // Print in exponential format
        println!("{:E}", fibo[idx % 3]);
        continue 'EachArg;
      }
    }

    // Print
    println!("{}", fibo[idx % 3]);
  }
}

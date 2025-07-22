use std::error::Error;

fn print_error(error_string: &str) {
  eprintln!("Usage: mygit {}", error_string);
  std::process::exit(1);
}

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    print_error("<command>");
  }

  // Must initialize a .mygit at the current directory to start tracking
  if args[1].as_str() != "init" && !std::path::Path::new(".mygit").exists() {
    eprintln!("Error: repository not initialized. Please run 'mygit init' first. ");
    std::process::exit(1);
  }

  // Match commands
  match args[1].as_str() {
    "init" => {
      if args.len() != 2 {
        print_error("init");
      }
      // Create .mygit directory to track history
      std::fs::create_dir_all(".mygit/objects")?;
      std::fs::create_dir_all(".mygit/refs/heads")?;
      std::fs::write(".mygit/HEAD", b"refs/heads/main\n")?;
      print!("Repository Initialized!")
    }

    // 1. Walk through working directory, skipping .mygit
    // 2. For each file, read its contents, compute content hash (sha-1), write raw bytes into .mygit/objects/<hash>
    // 3. Build a 'tree' object that records each path -> blob-hash mapping, serialize it, hash it, ans store it in objects/ too.
    // 4. Build a 'commit' object containing: tree hash, parent commit hash (read from refs/heads/main), author, timestamp, message
    // 5. Hash that commit object and write into objects/. then upate refs/head/main to point to new commit hash
    "commit" => {
      if args.len() != 3 {
        print_error("commit \"message\"");
      }
      let message = &args[2];
      println!("committing with message {}", message);


    }
    // Read the current commit hash from refs/heads/main
    // Loop through the entire list by setting hash = parent_hash, maybe stopping at 5? or just all of the commits
    "log" => {
      if args.len() != 2 {
        print_error("log");
      }
      println!("showing mygit history")
    }

    // Overwrite refs/heads/main with the target commit hash
    "reset" => {
      if args.len() != 3 {
        print_error("reset <# of versions>")
      }
      let steps = &args[2];
      println!("Resetting by {} versions", steps);
    }

    _ => {
      print_error("<init|commit|log|reset");
    }
  }
  // I can hold a dom version control system

  Ok(())
}


/**
 * grrs/src/main.rs
 */
use clap::Parser;

/// Search for a pattern in a file and display the line(s) containing it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// Give it a try: cargo run -- main src/main.rs should work now!
fn main() {
    let args = Cli::parse();
    // println!("{:?}", args.pattern);
    // println!("{:?}", args.path);
    // println!("Hello, world!");

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };
    println!("file content: {}", content);

    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}

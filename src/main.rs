use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Starting board state.
    #[arg(default_value = "0x2113_2113_4556_4786_900a")]
    start_image: String,
    /// Goal position mask for large pieces.
    #[arg(default_value = "0x0000_0000_0000_0ff0_0ff0")]
    goal_mask: String,
}

/// Runs the Klotski solver with the provided arguments.
fn main() {
    env_logger::init();
    let args = Args::parse();

    let rule =
        klotski::parse_args_to_rule(&args.start_image, &args.goal_mask).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        });

    klotski::run(std::io::stdout(), &rule).unwrap();
}

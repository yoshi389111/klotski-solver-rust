use clap::Parser;
use klotski::RuleError;

/// Command-line arguments for the Klotski solver.
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    run(&args)
}

/// Runs the Klotski solver with the provided arguments.
fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let rule = klotski::Rule::parse(&args.start_image, &args.goal_mask).unwrap_or_else(|e| {
        let message = convert_error_to_string(e);
        eprintln!("Error: {message}");
        std::process::exit(1);
    });

    let Some(path) = klotski::solve(&rule) else {
        println!("path not found.");
        return Ok(());
    };

    for (i, state) in path.iter().enumerate() {
        if let Some(piece) = state.piece {
            let p = &state.path;
            println!("step {i}: Move piece #{piece}: {p}");
        }
    }
    Ok(())
}

fn convert_error_to_string(e: RuleError) -> &'static str {
    match e {
        RuleError::InvalidStartBoardHexLength => "START_IMAGE must fit in 20 hex digits.",
        RuleError::StartBoardInvalidEmptyCount => "START_IMAGE must have only two empty spaces.",
        RuleError::FirstPieceMissingInStartBoard => "START_IMAGE must have the #1 large piece.",
        RuleError::InvalidPieceShape => "START_IMAGE contains an invalid piece shape.",
        RuleError::InvalidGoalMaskHexLength => "GOAL_MASK must fit in 20 hex digits.",
        RuleError::GoalMaskShapeError => "GOAL_MASK has an invalid shape.",
    }
}

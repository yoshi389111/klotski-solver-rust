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

fn convert_error_to_string(e: RuleError) -> String {
    match e {
        RuleError::InvalidStartBoardHexLength => {
            "START_IMAGE must be a 20 hex digit number.".to_string()
        }
        RuleError::StartBoardInvalidEmptySpaceCount => {
            "START_IMAGE must have only two empty spaces.".to_string()
        }
        RuleError::FirstPieceMissingInStartBoard => {
            "START_IMAGE must have the #1 large piece.".to_string()
        }
        RuleError::InvalidPieceShape => {
            "START_IMAGE contains a piece that is not of a legal shape.".to_string()
        }
        RuleError::InvalidGoalMaskHexLength => {
            "GOAL_MASK must be a 20 hex digit number.".to_string()
        }
        RuleError::GoalMaskSizeMismatch => {
            "GOAL_MASK must be a mask that indicates the goal position.".to_string()
        }
        RuleError::GoalMaskShapeError => "GOAL_MASK shape is incorrect.".to_string(),
    }
}

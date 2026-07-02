use clap::Parser;
use klotski::{Direction, Piece, Rule, RuleError, State, solve};

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
    let rule = Rule::parse(&args.start_image, &args.goal_mask).map_err(convert_error_to_str)?;

    if let Some(path) = solve(&rule) {
        path.iter()
            .enumerate()
            .for_each(|(i, state)| output_state(i, state));
    } else {
        println!("path not found.");
    };

    Ok(())
}

fn convert_error_to_str(e: RuleError) -> &'static str {
    match e {
        RuleError::InvalidStartBoardHexLength => "START_IMAGE must fit in 20 hex digits.",
        RuleError::InvalidStartBoardEmptyCount => "START_IMAGE must have only two empty spaces.",
        RuleError::MissingLargePiece => "START_IMAGE must have the #1 large piece.",
        RuleError::InvalidPieceShape => "START_IMAGE contains an invalid piece shape.",
        RuleError::InvalidGoalMaskHexLength => "GOAL_MASK must fit in 20 hex digits.",
        RuleError::InvalidGoalMaskEmptyCount => {
            "GOAL_MASK is an invalid mask for the goal positions."
        }
        RuleError::InvalidGoalMaskShape => "GOAL_MASK has an invalid shape.",
    }
}

fn output_state(i: usize, state: &State) {
    match state {
        State::Initial { .. } => {}
        State::SingleStep {
            piece, direction, ..
        } => {
            let piece = convert_piece_to_string(*piece);
            let direction = convert_direction_to_str(*direction);
            println!("step {i}: Move piece #{piece}: {direction}");
        }
        State::DoubleStep {
            piece,
            first_direction,
            second_direction,
            ..
        } => {
            let piece = convert_piece_to_string(*piece);
            let first_direction = convert_direction_to_str(*first_direction);
            let second_direction = convert_direction_to_str(*second_direction);
            println!("step {i}: Move piece #{piece}: {first_direction} and {second_direction}");
        }
    }
}

fn convert_direction_to_str(direction: Direction) -> &'static str {
    match direction {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    }
}

fn convert_piece_to_string(piece: Piece) -> String {
    format!("{:x}", piece.id)
}

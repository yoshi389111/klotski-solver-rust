mod bfs;
mod solver;

use solver::bit_pattern::BitPattern;
use solver::board::Board;
use solver::piece::Piece;
use solver::rule::Rule;

const SHAPE_UNUSE: BitPattern = BitPattern::new(0x0000_0000);
const SHAPE_SMALL: BitPattern = BitPattern::new(0x0000_000f);
const SHAPE_HORIZONTAL: BitPattern = BitPattern::new(0x0000_00ff);
const SHAPE_VERTICAL: BitPattern = BitPattern::new(0x000f_000f);
const SHAPE_LARGE: BitPattern = BitPattern::new(0x00ff_00ff);

pub fn run<W: std::io::Write>(mut output: W, rule: &Rule) -> std::io::Result<()> {
    let Some(path) = solver::solve(rule) else {
        writeln!(output, "path not found.")?;
        return Ok(());
    };

    for (i, state) in path.iter().enumerate() {
        if let Some(piece) = state.piece {
            let p = &state.path;
            writeln!(output, "step {i}: Move piece #{piece}: {p}")?;
        }
    }
    Ok(())
}

/// Parses the command line arguments to create a `Rule` object.
pub fn parse_args_to_rule(start_image: &str, goal_mask: &str) -> Result<Rule, KlotskiError> {
    let start_image = parse_20_hex_digits(start_image)
        .ok_or_else(|| KlotskiError::new("START_BOARD must be a 20 hex digit number."))?;

    if count_empty_spaces(&start_image) != 2 {
        return Err(KlotskiError::new(
            "START_BOARD must have only two empty spaces.",
        ));
    }

    if piece_shape(&start_image, 1) != SHAPE_LARGE {
        return Err(KlotskiError::new(
            "START_BOARD must have the #1 large piece.",
        ));
    }

    for i in 0x2u8..=0xf {
        let shape = piece_shape(&start_image, i);
        if !is_valid_regular_piece_shapes(&shape) {
            return Err(KlotskiError::new(
                "START_BOARD contains a piece that is not of a legal shape.",
            ));
        }
    }

    let goal_mask = parse_20_hex_digits(goal_mask)
        .ok_or_else(|| KlotskiError::new("GOAL_MASK must be a 20 hex digit number."))?;

    if count_empty_spaces(&goal_mask) != 16 {
        return Err(KlotskiError::new(
            "GOAL_MASK must be a mask that indicates the goal position.",
        ));
    }

    if piece_shape(&goal_mask, 0xf) != SHAPE_LARGE {
        return Err(KlotskiError::new("GOAL_MASK shape is incorrect."));
    }

    let rule = Rule::new(&Board::from_bitpattern(start_image), &goal_mask);

    Ok(rule)
}

/// Parses a string representing a 20 hex digit number, allowing for underscores as separators.
fn parse_20_hex_digits(value: &str) -> Option<BitPattern> {
    let value = value.trim_start_matches("0x").replace('_', "");
    match u128::from_str_radix(&value, 16) {
        Ok(n) if n <= 0xffff_ffff_ffff_ffff_ffff => Some(BitPattern::new(n)),
        _ => None,
    }
}

/// Returns the shape of the specified piece in the bit pattern.
fn piece_shape(bit_pattern: &BitPattern, piece_id: u8) -> BitPattern {
    let piece_mask: u128 = bit_pattern.mask_of(Piece::new(piece_id)).get_u128();
    let piece_shape = match piece_mask {
        0 => 0,
        _ => piece_mask >> piece_mask.trailing_zeros(),
    };
    BitPattern::new(piece_shape)
}

/// Checks if the given shape is a valid regular piece shape.
fn is_valid_regular_piece_shapes(shape: &BitPattern) -> bool {
    matches!(
        *shape,
        SHAPE_UNUSE | SHAPE_SMALL | SHAPE_HORIZONTAL | SHAPE_VERTICAL
    )
}

/// Counts the number of empty spaces in the given bit pattern.
fn count_empty_spaces(bit_pattern: &BitPattern) -> usize {
    let mut value = bit_pattern.get_u128();
    let mut count = 0;
    for _ in 0..20 {
        if (value & 0xf) == 0 {
            count += 1;
        }
        value >>= 4;
    }
    count
}

#[derive(Debug)]
pub enum KlotskiError {
    Validation(String),
}

impl KlotskiError {
    fn new(msg: &str) -> Self {
        KlotskiError::Validation(msg.into())
    }
}

impl std::fmt::Display for KlotskiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KlotskiError::Validation(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for KlotskiError {}

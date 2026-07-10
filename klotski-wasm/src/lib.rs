use klotski_core::{BfsSolver, Direction, KlotskiProblem, Piece, Rule, RuleError, State};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KlotskiSolver {
    solver: Option<BfsSolver<State, KlotskiProblem>>,
    result: Option<Vec<State>>,
}

#[wasm_bindgen]
impl KlotskiSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            solver: None,
            result: None,
        }
    }

    #[wasm_bindgen]
    pub fn set_start_board(&mut self, start_board: &str, goal_mask: &str) -> Result<(), String> {
        let rule =
            Rule::parse(start_board, goal_mask).map_err(|e| convert_error_to_str(e).to_string())?;
        self.solver = Some(KlotskiProblem::create_solver(rule));
        self.result = None;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn expand_level(&mut self) {
        if self.result.is_some() {
            return;
        }

        let Some(solver) = &mut self.solver else {
            return;
        };

        solver.expand_level();
        if solver.is_finished() {
            self.result = self
                .solver
                .take()
                .expect("solver was just checked")
                .find_path();
        }
    }

    #[wasm_bindgen]
    pub fn is_finished(&self) -> bool {
        self.solver.is_none()
    }

    #[wasm_bindgen]
    pub fn get_result(&mut self) -> String {
        let Some(path) = &self.result else {
            return "path not found.".to_string();
        };

        path.iter()
            .enumerate()
            .map(|(i, state)| state_to_string(i, state))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Default for KlotskiSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn solve(start_board: &str, goal_mask: &str) -> String {
    let rule = match Rule::parse(start_board, goal_mask) {
        Ok(rule) => rule,
        Err(e) => return convert_error_to_str(e).to_string(),
    };

    let solver = KlotskiProblem::create_solver(rule);

    let Some(path) = solver.find_path() else {
        return "path not found.".to_string();
    };

    path.iter()
        .enumerate()
        .map(|(i, state)| state_to_string(i, state))
        .collect::<Vec<String>>()
        .join("\n")
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

fn state_to_string(i: usize, state: &State) -> String {
    match state {
        State::Initial { .. } => "".to_string(),
        State::SingleStep {
            piece, direction, ..
        } => {
            let piece = convert_piece_to_string(*piece);
            let direction = convert_direction_to_str(*direction);
            format!("step {i}: Move piece #{piece}: {direction}")
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
            format!("step {i}: Move piece #{piece}: {first_direction} and {second_direction}")
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

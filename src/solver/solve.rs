use super::BoardKey;
use super::MovePath;
use super::Rule;
use super::State;
use super::VisitedHistory;
use crate::bfs;

/// Solves the klotski puzzle using a breadth-first search algorithm.
pub fn solve(rule: &Rule) -> Option<Vec<State>> {
    let start_state = State {
        board: rule.start.clone(),
        piece: None,
        path: MovePath::None,
    };

    let is_goal = |s: &State| rule.is_finished(&s.board);
    let neighbors = |s: &State| s.get_neighbors(rule);

    let mut visited = VisitedHistory::new();
    let try_visit =
        |s: &State, depth: usize| visited.try_visit(BoardKey::create(rule, &s.board), depth);

    bfs::find_path(&start_state, is_goal, neighbors, try_visit)
}

#[cfg(test)]
mod tests {
    use super::super::BitPattern;
    use super::super::Board;
    use super::*;

    #[test]
    fn test_solve_returns_none_for_unsolvable() {
        // Arrange: Test solve returns None for unsolvable puzzle
        let rule = Rule::new(
            &Board::new(0x2112_2112_3344_5678_5008),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        // Act
        let result = solve(&rule);
        // Assert
        assert_eq!(result, None);
    }
}

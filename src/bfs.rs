mod path_finder;

use path_finder::PathFinder;

/// Finds a path from the start state to a goal state using a breadth-first search algorithm.
///
/// - `start_state` is the initial state.
/// - `is_goal` is a function that checks if a given state is the goal state.
/// - `neighbors` is a function that returns the next states of a given state.
/// - `try_visit` is a function that takes a state and the current depth, and returns `true` if the state should be visited (i.e., it is unvisited), or `false` otherwise.
///
/// Returns an `Option<Vec<T>>` containing the path from the start state to the goal state if found, or `None` if no path exists.
pub fn find_path<T, FGoal, FNext, FVisit>(
    start_state: &T,
    is_goal: FGoal,
    neighbors: FNext,
    try_visit: FVisit,
) -> Option<Vec<T>>
where
    T: Clone,
    FGoal: Fn(&T) -> bool,
    FNext: Fn(&T) -> Vec<T>,
    FVisit: FnMut(&T, usize) -> bool,
{
    let mut finder = PathFinder::new(is_goal, neighbors, try_visit);
    finder.find(start_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_path_linear() {
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
        // 0 -> 1 -> 2 -> 3 -> 4
        let neighbors = |&x: &i32| if x < goal { vec![x + 1] } else { vec![] };
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_find_path_branch() {
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
        // 0 -> 1 -> 3 or
        // 0 -> 2 -> 4
        let neighbors = |&x: &i32| match x {
            0 => vec![1, 2],
            1 => vec![3],
            2 => vec![4],
            _ => vec![],
        };
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0, 2, 4]));
    }

    #[test]
    fn test_find_path_shortest() {
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
        // 0 -> 1 -> 4 or
        // 0 -> 2 -> 3 -> 4
        let neighbors = |&x: &i32| match x {
            0 => vec![1, 2],
            1 => vec![4],
            _ => vec![x + 1],
        };
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0, 1, 4]));
    }

    #[test]
    fn test_find_path_revisit() {
        let start = 0;
        let goal = 5;
        let is_goal = |&x: &i32| x == goal;
        let neighbors = |&x: &i32| vec![x + 1, x - 1];
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_find_path_not_found() {
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
        // 0 -> 1 -> 2
        let neighbors = |&x: &i32| match x {
            0 => vec![1],
            1 => vec![2],
            _ => vec![],
        };
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, None);
    }

    #[test]
    fn test_find_path_alread_goaled() {
        let start = 0;
        let goal = 0;
        let is_goal = |&x: &i32| x == goal;
        let neighbors = |&x: &i32| vec![x + 1];
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0]));
    }
}

use std::collections::VecDeque;
use std::rc::Rc;

// --- Traits ---

pub trait TracePath<T> {
    fn trace_path(&self) -> Vec<T>;
}

// --- Structs ---

#[derive(Debug)]
struct Node<T> {
    state: T,
    parent: Option<Rc<Node<T>>>,
}

// --- Trait Implementations ---

impl<T: Clone> TracePath<T> for Rc<Node<T>> {
    /// Returns the path from the start state to the goal state.
    fn trace_path(&self) -> Vec<T> {
        let mut path = Vec::new();
        let mut node_opt = Some(self.clone());
        while let Some(node) = node_opt {
            path.push(node.state.clone());
            node_opt = node.parent.clone();
        }
        path.reverse();
        path
    }
}

// --- Functions ---

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
    mut try_visit: FVisit,
) -> Option<Vec<T>>
where
    T: Clone,
    FGoal: Fn(&T) -> bool,
    FNext: Fn(&T) -> Vec<T>,
    FVisit: FnMut(&T, usize) -> bool,
{
    let mut queue = VecDeque::new();

    const START_DEPTH: usize = 0;
    if (try_visit)(start_state, START_DEPTH) {
        let start_node = Rc::new(Node {
            state: start_state.clone(),
            parent: None,
        });
        if (is_goal)(start_state) {
            return Some(start_node.trace_path()); // Found immediately.
        }
        queue.push_back((start_node, START_DEPTH));
    }

    while let Some((current_node, current_depth)) = queue.pop_front() {
        let next_depth = current_depth + 1;
        for next_state in (neighbors)(&current_node.state) {
            if (try_visit)(&next_state, next_depth) {
                let next_node = Rc::new(Node {
                    state: next_state.clone(),
                    parent: Some(current_node.clone()),
                });
                if (is_goal)(&next_state) {
                    return Some(next_node.trace_path()); // Found.
                }
                queue.push_back((next_node, next_depth));
            }
        }
    }
    None // Not Found.
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_path_linear() {
        // Linear path: 0 -> 1 -> 2 -> 3 -> 4
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
        let neighbors = |&x: &i32| if x < goal { vec![x + 1] } else { vec![] };
        let mut visited = HashSet::new();
        let try_visit = |x: &i32, _depth: usize| visited.insert(*x);

        let path = find_path(&start, is_goal, neighbors, try_visit);
        assert_eq!(path, Some(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_find_path_branch() {
        // Branching path: 0 -> 1 -> 3 or 0 -> 2 -> 4
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
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
        // Shortest path: 0 -> 1 -> 4 (not 0 -> 2 -> 3 -> 4)
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
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
        // Path with possible revisits: 0 -> 1 -> 2 -> 3 -> 4 -> 5
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
        // No path to goal
        let start = 0;
        let goal = 4;
        let is_goal = |&x: &i32| x == goal;
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
        // Start is already goal
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

use std::collections::VecDeque;
use std::rc::Rc;

/// Represents a node in the search tree, containing the state and a reference to its previous node.
#[derive(Debug)]
struct Node<T> {
    state: T,
    previous: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    /// Creates a new `Node` with the given state and previous node.
    fn new(state: T, previous: Option<Rc<Node<T>>>) -> Rc<Self> {
        Rc::new(Node { state, previous })
    }
}

/// A trait representing a search problem that can be solved using breadth-first search.
pub trait SearchProblem<T: Clone> {
    /// Attempts to visit the given state at the specified depth.
    fn try_visit(&mut self, state: &T, depth: usize) -> bool;

    /// Checks if the given state is a goal state.
    fn is_goal(&self, state: &T) -> bool;

    /// Returns the neighboring states of the given state.
    fn neighbors(&self, state: &T) -> Vec<T>;
}

/// A breadth-first search solver for a given search problem.
#[derive(Debug)]
pub struct BfsSolver<T, V>
where
    T: Clone,
    V: SearchProblem<T>,
{
    /// The search problem to be solved.
    problem: V,
    /// The queue of nodes to be explored.
    queue: VecDeque<Rc<Node<T>>>,
    /// The resulting path from the start state to the goal state, if found.
    result_path: Option<Vec<T>>,
    /// The current depth level of the search.
    depth: usize,
}

impl<T: Clone, V: SearchProblem<T>> BfsSolver<T, V> {
    /// Creates a new `BfsSolver`.
    pub fn new(start_state: &T, mut problem: V) -> Self {
        let mut queue = VecDeque::new();
        let result_path = if problem.try_visit(start_state, 0) && problem.is_goal(start_state) {
            Some(vec![start_state.clone()])
        } else {
            queue.push_back(Node::new(start_state.clone(), None));
            None
        };

        BfsSolver {
            queue,
            problem,
            result_path,
            depth: 0,
        }
    }

    /// Finds a path from the start state to a goal state using a breadth-first search algorithm.
    pub fn find_path(mut self) -> Option<Vec<T>> {
        while !self.is_finished() {
            self.expand_level();
        }
        self.into_path()
    }

    /// Expands the current level of the search tree.
    pub fn expand_level(&mut self) {
        if self.is_finished() {
            return;
        }
        let mut next_queue = VecDeque::new();
        self.depth += 1;
        while let Some(current_node) = self.queue.pop_front() {
            for next_state in self.problem.neighbors(&current_node.state) {
                if !self.problem.try_visit(&next_state, self.depth) {
                    // Already visited.
                    continue;
                }
                let next_node = Node::new(next_state.clone(), Some(current_node.clone()));
                if self.problem.is_goal(&next_state) {
                    // Found the goal state.
                    self.result_path = Some(Self::trace_path(next_node));
                    self.queue.clear();
                    return;
                }
                next_queue.push_back(next_node);
            }
        }
        self.queue = next_queue;
    }

    /// Returns the path from the start state to the goal state.
    fn trace_path(node: Rc<Node<T>>) -> Vec<T> {
        let mut path = Vec::new();
        let mut current_node = Some(node);
        while let Some(node) = current_node {
            path.push(node.state.clone());
            current_node = node.previous.clone();
        }
        path.reverse();
        path
    }

    /// Checks if the search has finished.
    pub fn is_finished(&self) -> bool {
        self.result_path.is_some() || self.queue.is_empty()
    }

    /// Returns the path from the start state to the goal state if found, or `None` if no path exists.
    pub fn into_path(self) -> Option<Vec<T>> {
        self.result_path
    }

    /// Returns the current depth level of the search.
    #[allow(dead_code)]
    pub fn depth_level(&self) -> usize {
        self.depth
    }

    /// Returns the current size of the queue.
    #[allow(dead_code)]
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_path_linear() {
        // Arrange: Linear path: 0 -> 1 -> 2 -> 3 -> 4
        const GOAL: i32 = 4;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                if *state < GOAL {
                    vec![*state + 1]
                } else {
                    vec![]
                }
            }
        }
        let start = 0;

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, Some(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_find_path_branch() {
        // Arrange: Branching path: 0 -> 1 -> 3 or 0 -> 2 -> 4
        let start = 0;
        const GOAL: i32 = 4;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                match *state {
                    0 => vec![1, 2],
                    1 => vec![3],
                    2 => vec![4],
                    _ => vec![],
                }
            }
        }

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, Some(vec![0, 2, 4]));
    }

    #[test]
    fn test_find_path_shortest() {
        // Arrange: Shortest path: 0 -> 1 -> 4 (not 0 -> 2 -> 3 -> 4)
        let start = 0;
        const GOAL: i32 = 4;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                match *state {
                    0 => vec![1, 2],
                    1 => vec![4],
                    _ => vec![*state + 1],
                }
            }
        }

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, Some(vec![0, 1, 4]));
    }

    #[test]
    fn test_find_path_revisit() {
        // Arrange: Path with possible revisits: 0 -> 1 -> 2 -> 3 -> 4 -> 5
        let start = 0;
        const GOAL: i32 = 5;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                vec![*state + 1, *state - 1]
            }
        }

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, Some(vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_find_path_not_found() {
        // Arrange: No path to goal
        let start = 0;
        const GOAL: i32 = 4;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                match *state {
                    0 => vec![1],
                    1 => vec![2],
                    _ => vec![],
                }
            }
        }

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, None);
    }

    #[test]
    fn test_find_path_alread_goaled() {
        // Arrange: Start is already goal
        let start = 0;
        const GOAL: i32 = 0;
        struct TestVisitPolicy {
            visited: HashSet<i32>,
        }
        impl TestVisitPolicy {
            fn new() -> Self {
                Self {
                    visited: HashSet::new(),
                }
            }
        }
        impl SearchProblem<i32> for TestVisitPolicy {
            fn try_visit(&mut self, state: &i32, _depth: usize) -> bool {
                self.visited.insert(*state)
            }

            fn is_goal(&self, state: &i32) -> bool {
                *state == GOAL
            }

            fn neighbors(&self, state: &i32) -> Vec<i32> {
                vec![*state + 1]
            }
        }

        // Act
        let path = BfsSolver::new(&start, TestVisitPolicy::new()).find_path();

        // Assert
        assert_eq!(path, Some(vec![0]));
    }
}

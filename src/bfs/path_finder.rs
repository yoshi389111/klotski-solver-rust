use std::collections::VecDeque;
use std::rc::Rc;

/// A structure representing a path finder that uses a breadth-first search algorithm to find paths in a state space.
#[derive(Debug)]
pub struct PathFinder<T, FGoal, FNext, FVisit>
where
    T: Clone,
    FGoal: Fn(&T) -> bool,
    FNext: Fn(&T) -> Vec<T>,
    FVisit: FnMut(&T, usize) -> bool,
{
    queue: VecDeque<(Rc<Node<T>>, usize)>,
    is_goal: FGoal,
    neighbors: FNext,
    try_visit: FVisit,
}

impl<T, FGoal, FNext, FVisit> PathFinder<T, FGoal, FNext, FVisit>
where
    T: Clone,
    FGoal: Fn(&T) -> bool,
    FNext: Fn(&T) -> Vec<T>,
    FVisit: FnMut(&T, usize) -> bool,
{
    /// Creates a new `PathFinder` instance.
    ///
    /// # Arguments
    /// * `is_goal`: A closure that checks if a state is the goal.
    /// * `neighbors`: A closure that returns the neighboring states of a given state.
    /// * `try_visit`: A closure that takes a state and the current depth, and returns `true` if the state should be visited (i.e., it is unvisited), or `false` otherwise.
    /// # Returns
    /// A new `PathFinder` instance.
    pub fn new(is_goal: FGoal, neighbors: FNext, try_visit: FVisit) -> Self {
        Self {
            queue: VecDeque::new(),
            is_goal,
            neighbors,
            try_visit,
        }
    }

    /// Finds a path from the start state to the goal state.
    ///
    /// # Arguments
    /// * `start_state`: The initial state from which to start the search.
    /// # Returns
    /// An `Option<Vec<T>>` containing the path from the start state to the goal state if found, or `None` if no path exists.
    pub fn find(&mut self, start_state: &T) -> Option<Vec<T>> {
        const START_DEPTH: usize = 0;
        if self.should_visit(start_state, START_DEPTH) {
            let start_node = self.make_node(start_state, None);
            if self.is_goal_state(start_state) {
                return Some(start_node.trace_path()); // Found immediately.
            }
            self.enqueue(start_node, START_DEPTH);
        }

        while let Some((current_node, current_depth)) = self.dequeue() {
            let next_depth = current_depth + 1;
            for next_state in (self.neighbors)(&current_node.state) {
                if self.should_visit(&next_state, next_depth) {
                    let next_node = self.make_node(&next_state, Some(current_node.clone()));
                    if self.is_goal_state(&next_state) {
                        return Some(next_node.trace_path()); // Found.
                    }
                    self.enqueue(next_node, next_depth);
                }
            }
        }
        None // Not Found.
    }

    fn should_visit(&mut self, state: &T, depth: usize) -> bool {
        (self.try_visit)(state, depth)
    }

    fn is_goal_state(&self, state: &T) -> bool {
        (self.is_goal)(state)
    }

    fn enqueue(&mut self, node: Rc<Node<T>>, depth: usize) {
        self.queue.push_back((node, depth));
    }

    fn dequeue(&mut self) -> Option<(Rc<Node<T>>, usize)> {
        self.queue.pop_front()
    }

    fn make_node(&self, state: &T, parent: Option<Rc<Node<T>>>) -> Rc<Node<T>> {
        Rc::new(Node {
            state: state.clone(),
            parent,
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    state: T,
    parent: Option<Rc<Node<T>>>,
}

pub trait TracePath<T> {
    fn trace_path(&self) -> Vec<T>;
}

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
